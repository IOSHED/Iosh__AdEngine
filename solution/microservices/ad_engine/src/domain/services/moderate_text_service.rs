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

        Ok(text
            .iter()
            .map(|original_str| self.mask_abusive_words(&self.filter_phrase(original_str.clone()), &abusive_words))
            .collect())
    }

    fn mask_abusive_words(&self, original_str: &str, abusive_words: &[String]) -> String {
        let words: Vec<&str> = original_str.split_whitespace().collect();

        words
            .iter()
            .map(|&word| self.mask_word(word, abusive_words))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn mask_word(&self, word: &str, abusive_words: &[String]) -> String {
        let cleaned_word = word.to_lowercase().replace(' ', "");

        if abusive_words
            .par_iter()
            .any(|abusive_word| self.is_abusive_word(&cleaned_word, abusive_word))
        {
            "***".to_string()
        } else {
            word.to_string()
        }
    }

    pub fn is_abusive_word(&self, cleaned_word: &str, abusive_word: &str) -> bool {
        let word_length = abusive_word.len();
        let cleaned_word_length = cleaned_word.len();

        if word_length == 0 || word_length > cleaned_word_length {
            return false;
        }

        (0..=cleaned_word_length.saturating_sub(word_length)).any(|part| {
            let fragment: String = cleaned_word.chars().skip(part).take(word_length).collect();
            self.levenshtein_distance(&fragment, abusive_word)
                <= (word_length as f32 * self.sensitivity).round() as usize
        })
    }

    pub async fn check_abusive_content<R: IGetAbusiveWords>(
        &self,
        text: &[String],
        is_activated: bool,
        repo: R,
    ) -> domain::services::ServiceResult<bool> {
        if !is_activated {
            return Ok(false);
        }

        let abusive_words = repo
            .get_words()
            .await
            .map_err(domain::services::ServiceError::Repository)?;

        for original_str in text {
            let filtered_str = self.filter_phrase(original_str.clone());
            if self.contains_abusive_words(&filtered_str, &abusive_words) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn contains_abusive_words(&self, original_str: &str, abusive_words: &[String]) -> bool {
        let words: Vec<&str> = original_str.split_whitespace().collect();

        words.iter().any(|&word| self.is_abusive_word_vec(word, abusive_words))
    }

    fn is_abusive_word_vec(&self, word: &str, abusive_words: &[String]) -> bool {
        let cleaned_word = word.to_lowercase().replace(' ', "");

        abusive_words.iter().any(|abusive_word| {
            self.levenshtein_distance(&cleaned_word, abusive_word)
                <= (abusive_word.len() as f32 * self.sensitivity).round() as usize
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRepo {
        words: Vec<String>,
    }

    #[async_trait]
    impl IGetAbusiveWords for MockRepo {
        async fn get_words(&self) -> infrastructure::repository::RepoResult<Vec<String>> {
            Ok(self.words.clone())
        }
    }

    #[test]
    fn test_mask_abusive_words() {
        let service = ModerateTextService::new(0.2);
        let abusive_words = vec!["плохое".to_string()];

        assert_eq!(
            service.mask_abusive_words("Это не плохое", &abusive_words),
            "Это не ***"
        );
        assert_eq!(
            service.mask_abusive_words("Это не хорошее да да", &abusive_words),
            "Это не хорошее да да"
        );
    }

    #[test]
    fn test_is_abusive_word() {
        let service = ModerateTextService::new(0.3);

        assert!(service.is_abusive_word("плохо", "плохо"));
        assert!(service.is_abusive_word("плохое", "плохо"));
        assert!(!service.is_abusive_word("плоховастенький", "плохо"));
    }

    #[tokio::test]
    async fn test_hide_abusive_content() {
        let service = ModerateTextService::new(0.2);
        let repo = MockRepo {
            words: vec!["плохо".to_string()],
        };
        let text = vec![
            "Это, не хорошо а пло][о .выспасть".to_string(),
            "Это ю хороше .но не".to_string(),
        ];

        let result = service.hide_abusive_content(&text, true, repo).await.unwrap();

        assert_eq!(result, vec!["Это, не хорошо а *** .выспасть", "Это ю хороше .но не"]);
    }

    #[tokio::test]
    async fn test_check_abusive_content() {
        let service = ModerateTextService::new(0.3);
        let repo = MockRepo {
            words: vec!["плохо".to_string()],
        };
        let text = vec!["Не очнеь плоhо".to_string(), "Хя хя нет п".to_string()];

        let result = service.check_abusive_content(&text, true, repo).await.unwrap();

        assert!(result);

        let repo = MockRepo {
            words: vec!["плохо".to_string()],
        };

        let clean_text = vec!["Хях я нет".to_string()];
        let result_clean = service.check_abusive_content(&clean_text, true, repo).await.unwrap();

        assert!(!result_clean);
    }

    #[tokio::test]
    async fn test_check_abusive_content_disabled() {
        let service = ModerateTextService::new(0.3);
        let repo = MockRepo {
            words: vec!["badword".to_string()],
        };
        let text = vec!["This is a badword".to_string()];

        let result = service.check_abusive_content(&text, false, repo).await.unwrap();

        assert!(!result);
    }
}
