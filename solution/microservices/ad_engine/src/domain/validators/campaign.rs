use crate::domain;

async fn validate_start_date(start_date: u32, time_advance: u32) -> Result<(), domain::services::ServiceError> {
    if start_date < time_advance {
        return Err(domain::services::ServiceError::Validation(
            "start_date must be more or equal to time_advance".into(),
        ));
    }
    Ok(())
}

async fn validate_date_range(start_date: u32, end_date: u32) -> Result<(), domain::services::ServiceError> {
    if start_date > end_date {
        return Err(domain::services::ServiceError::Validation(
            "start_date must be under or equal to end_date".into(),
        ));
    }
    Ok(())
}

async fn validate_age_range(age_from: Option<u8>, age_to: Option<u8>) -> Result<(), domain::services::ServiceError> {
    if age_from.is_none() || age_to.is_none() {
        return Ok(());
    }
    if age_from > age_to {
        return Err(domain::services::ServiceError::Validation(
            "age_from must be under or equal to age_to".into(),
        ));
    }
    Ok(())
}

async fn validate_limits_range(
    impressions_limit: u32,
    clicks_limit: u32,
) -> Result<(), domain::services::ServiceError> {
    if clicks_limit > impressions_limit {
        return Err(domain::services::ServiceError::Validation(
            "clicks_limit must be under or equal to impressions_limit".into(),
        ));
    }
    Ok(())
}

pub async fn validate_campaign_data(
    start_date: u32,
    end_date: u32,
    age_from: Option<u8>,
    age_to: Option<u8>,
    impressions_limit: u32,
    clicks_limit: u32,
    time_advance: u32,
) -> Result<(), domain::services::ServiceError> {
    validate_start_date(start_date, time_advance).await?;
    validate_date_range(start_date, end_date).await?;
    validate_age_range(age_from, age_to).await?;
    validate_limits_range(impressions_limit, clicks_limit).await
}
