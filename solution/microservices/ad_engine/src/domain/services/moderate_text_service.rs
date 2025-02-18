use async_trait::async_trait;
use rayon::prelude::*;

use crate::{domain, infrastructure};

#[async_trait]
pub trait IGetAbusiveWords {
    async fn get_words(&self) -> infrastructure::repository::RepoResult<Vec<String>>;
}

#[derive(Debug)]
pub struct ModerateTextService {
    sensitivity: f32,
}

impl ModerateTextService {
    pub fn new(sensitivity: f32) -> Self {
        Self { sensitivity }
    }
}

impl ModerateTextService {
    pub async fn hide_abusive_content<R: IGetAbusiveWords>(
        &self,
        text: &[String],
        is_activated: bool,
        repo: R,
    ) -> domain::services::ServiceResult<Vec<String>> {
        if !is_activated {
            return Ok(text.to_vec());
        }
        let abusive_words = repo
            .get_words()
            .await
            .map_err(domain::services::ServiceError::Repository)?;

        let result = text
            .iter()
            .map(|original_str| {
                let phrase = original_str.to_lowercase().replace(' ', "");
                let filtered_phrase = self.filter_phrase(phrase);

                let phrase_length = filtered_phrase.len();

                let has_abusive_content = abusive_words.par_iter().any(|word| {
                    let word_length = word.len();
                    if word_length == 0 || word_length > phrase_length {
                        return false;
                    }

                    (0..=phrase_length.saturating_sub(word_length)).any(|part| {
                        let fragment: String = filtered_phrase.chars().skip(part).take(word_length).collect();
                        self.levenshtein_distance(&fragment, word)
                            <= (word_length as f32 * self.sensitivity).round() as usize
                    })
                });

                if has_abusive_content {
                    "***".to_string()
                } else {
                    original_str.clone()
                }
            })
            .collect();

        Ok(result)
    }

    pub async fn check_abusive_content<R: IGetAbusiveWords>(
        &self,
        text: &[String],
        is_activated: bool,
        repo: R,
    ) -> domain::services::ServiceResult<()> {
        if !is_activated {
            return Ok(());
        }
        let abusive_words = repo
            .get_words()
            .await
            .map_err(|e| domain::services::ServiceError::Repository(e))?;

        let phrase: String = text.join(" ").to_lowercase().replace(" ", "");
        let filtered_phrase = self.filter_phrase(phrase);

        let phrase_length = filtered_phrase.len();

        let result = abusive_words.par_iter().find_map_any(|word| {
            let word_length = word.len();
            if word_length == 0 || word_length > phrase_length {
                return None;
            }

            (0..=phrase_length.saturating_sub(word_length)).find_map(|part| {
                let fragment: String = filtered_phrase.chars().skip(part).take(word_length).collect();
                if self.levenshtein_distance(&fragment, word)
                    <= (word_length as f32 * self.sensitivity).round() as usize
                {
                    Some(word)
                } else {
                    None
                }
            })
        });

        if let Some(res) = result {
            return Err(domain::services::ServiceError::Censorship(res.into()));
        }
        Ok(())
    }

    fn filter_phrase(&self, phrase: String) -> String {
        let substitution_map = self.get_map_symbol();
        let mut filtered_phrase = phrase;

        for (key, values) in substitution_map.as_object().unwrap() {
            for letter in values.as_array().unwrap() {
                let letter_str = letter.as_str().unwrap();
                filtered_phrase = filtered_phrase.replace(letter_str, key);
            }
        }
        filtered_phrase
    }

    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let n = a.len();
        let m = b.len();
        let mut current_row: Vec<usize> = (0..=n).collect();

        for i in 1..=m {
            let previous_row = current_row.clone();
            current_row[0] = i;

            for j in 1..=n {
                let add = previous_row[j] + 1;
                let delete = current_row[j - 1] + 1;
                let change = if a.chars().nth(j - 1) != b.chars().nth(i - 1) {
                    previous_row[j - 1] + 1
                } else {
                    previous_row[j - 1]
                };

                current_row[j] = add.min(delete).min(change);
            }
        }
        current_row[n]
    }

    fn get_map_symbol(&self) -> serde_json::Value {
        serde_json::json!({
                "а": ["а", "a", "@"],
                "б": ["б", "6", "b"],
                "в": ["в", "b", "v"],
                "г": ["г", "r", "g"],
                "д": ["д", "d"],
                "е": ["е", "e"],
                "ё": ["ё", "e"],
                "ж": ["ж", "zh", "*"],
                "з": ["з", "3", "z"],
                "и": ["и", "u", "i"],
                "й": ["й", "u", "i"],
                "к": ["к", "k", "i{", "|{"],
                "л": ["л", "l", "ji"],
                "м": ["м", "m"],
                "н": ["н", "h", "n"],
                "о": ["о", "o", "0"],
                "п": ["п", "n", "p"],
                "р": ["р", "r", "p"],
                "с": ["с", "c", "s"],
                "т": ["т", "m", "t"],
                "у": ["у", "y", "u"],
                "ф": ["ф", "f"],
                "х": ["х", "x", "h", "}{", "]["],
                "ц": ["ц", "c", "u,"],
                "ч": ["ч", "ch"],
                "ш": ["ш", "sh"],
                "щ": ["щ", "sch"],
                "ь": ["ь", "b"],
                "ы": ["ы", "bi"],
                "ъ": ["ъ"],
                "э": ["э", "e"],
                "ю": ["ю", "io"],
                "я": ["я", "ya"]
            }
        )
    }
}
