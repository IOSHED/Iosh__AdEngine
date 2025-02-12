use async_trait::async_trait;

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct PgUserRepository<'p> {
    pg_pool: &'p sqlx::Pool<sqlx::Postgres>,
}

impl<'p> PgUserRepository<'p> {
    pub fn new(pg_pool: &'p sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pg_pool }
    }
}

#[derive(Debug)]
pub struct UserReturningSchema {
    pub telegram_id: i64,
    pub birth_day: chrono::NaiveDate,
    pub city: String,
    pub bio: Option<String>,
    pub interests: Vec<String>, 
    pub country_code: String,
}

#[async_trait]
impl<'p> domain::services::repository::IRegisterUser for PgUserRepository<'p> {
    async fn register(
        &self,
        telegram_id: i32, 
        birth_day: chrono::NaiveDate,
        latitude: f64,
        longitude: f64,
        city: String,
        country_code: String,
        bio: Option<String>,
        interests: Vec<String>,
    ) -> infrastructure::repository::RepoResult<UserReturningSchema> {
        let mut transaction = self.pg_pool.begin().await.map_err(|_| infrastructure::repository::RepoError::Unknown)?;
        
        let user_result = sqlx::query_as!(
            UserReturningSchema,
            "
            WITH inserted_user AS(
                INSERT INTO users (telegram_id) 
                VALUES ($1)
                RETURNING id, telegram_id
            ), inserted_user_profile AS (
                INSERT INTO users_profile (birth_day, latitude, longitude, city, bio, interests, user_id, country_code_id)
                VALUES ($2, $3, $4, $5, $6, $7, (SELECT id FROM inserted_user), (SELECT id FROM countries WHERE UPPER(alpha2) = UPPER($8)))
                RETURNING birth_day, city, bio, interests, country_code_id, user_id
            )
            SELECT iu.telegram_id, iup.birth_day, iup.city, iup.bio, iup.interests, c.alpha2 AS country_code
            FROM inserted_user_profile AS iup
            JOIN countries AS c ON iup.country_code_id = c.id
            JOIN users AS iu ON iup.user_id = iu.id;
            ",
            telegram_id,
            birth_day,
            latitude,
            longitude,
            city,
            bio,
            &interests,
            country_code,
        )
        .fetch_one(&mut *transaction)
        .await;
        
        match user_result {
            Ok(user) => {
                transaction.commit().await.map_err(|_| infrastructure::repository::RepoError::Unknown)?;
                Ok(user)
            }
            Err(e) => {
                transaction.rollback().await.map_err(|_| infrastructure::repository::RepoError::Unknown)?;
                if e.to_string().contains("null value in column \"country_code_id\"") {
                    return Err(infrastructure::repository::RepoError::ObjDoesNotExists("country code".to_string()));
                }
                return Err(infrastructure::repository::RepoError::Unknown);
            }
        }
    }
}

#[async_trait]
impl<'p> domain::services::repository::IUserAreExists for PgUserRepository<'p> {
    async fn are_exist(
        &self,
        telegram_id: i32,
    ) -> infrastructure::repository::RepoResult<bool> {
        Ok(sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE telegram_id = $1) as exists",
            telegram_id
        )
        .fetch_one(self.pg_pool)
        .await
        .map(|row| row.exists.unwrap_or(false))
        .unwrap_or(false))
    }
}
