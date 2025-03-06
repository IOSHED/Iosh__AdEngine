use async_trait::async_trait;
use rayon::prelude::*;

use crate::{domain, infrastructure};

/// Trait for retrieving abusive words from a data source.
#[async_trait]
pub trait IGetAbusiveWords {
    /// Retrieves the list of abusive words.
    ///
    /// # Returns
    /// A `RepoResult` containing a vector of abusive words as strings.
    async fn get_words(&self) -> infrastructure::repository::RepoResult<Vec<String>>;
}

/// Service for moderating and filtering abusive content from text.
///
/// This service provides functionality to detect and mask abusive words in text
/// content, with configurable sensitivity for fuzzy matching.
#[derive(Debug)]
pub struct ModerateTextService {
    /// Sensitivity threshold for fuzzy matching of abusive words (0.0 to 1.0).
    /// Higher values allow more variations of words to be matched.
    sensitivity: f32,
}

impl ModerateTextService {
    /// Creates a new `ModerateTextService` with the specified sensitivity.
    ///
    /// # Arguments
    /// * `sensitivity` - Fuzzy matching sensitivity threshold (0.0 to 1.0)
    ///
    /// # Returns
    /// A new instance of `ModerateTextService`
    pub fn new(sensitivity: f32) -> Self {
        Self { sensitivity }
    }
}

impl ModerateTextService {
    /// Masks abusive content in the provided text by replacing offensive words
    /// with asterisks.
    ///
    /// # Arguments
    /// * `text` - Slice of strings to check for abusive content
    /// * `is_activated` - Whether content moderation is enabled
    /// * `repo` - Repository implementing `IGetAbusiveWords` to fetch abusive
    ///   word list
    ///
    /// # Returns
    /// A `ServiceResult` containing a vector of strings with abusive content
    /// masked
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

    /// Masks abusive words in a single string with asterisks.
    ///
    /// # Arguments
    /// * `original_str` - String to check for abusive content
    /// * `abusive_words` - List of abusive words to match against
    ///
    /// # Returns
    /// String with abusive words replaced by "***"
    fn mask_abusive_words(&self, original_str: &str, abusive_words: &[String]) -> String {
        let words: Vec<&str> = original_str.split_whitespace().collect();

        words
            .iter()
            .map(|&word| self.mask_word(word, abusive_words))
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Masks a single word if it matches any abusive word.
    ///
    /// # Arguments
    /// * `word` - Word to check
    /// * `abusive_words` - List of abusive words to match against
    ///
    /// # Returns
    /// Either "***" if word is abusive, or the original word
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

    /// Checks if a word matches an abusive word within the sensitivity
    /// threshold.
    ///
    /// # Arguments
    /// * `cleaned_word` - Normalized word to check
    /// * `abusive_word` - Abusive word to match against
    ///
    /// # Returns
    /// `true` if the word matches within sensitivity threshold, `false`
    /// otherwise
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

    /// Checks if text contains any abusive content by comparing against a list
    /// of prohibited words.
    ///
    /// Performs fuzzy matching with configurable sensitivity to catch common
    /// misspellings and character substitutions. Processes text in parallel
    /// for better performance.
    ///
    /// # Arguments
    /// * `text` - Slice of strings to analyze for abusive content
    /// * `is_activated` - Flag to enable/disable content moderation
    /// * `repo` - Repository that provides the list of prohibited words
    ///
    /// # Returns
    /// A `ServiceResult` that is:
    /// - `Ok(true)` if abusive content is detected
    /// - `Ok(false)` if no abusive content is found
    /// - `Err(ServiceError::Censorship)` containing the first detected abusive
    ///   word
    /// - `Err(ServiceError::Repository)` if fetching prohibited words fails
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

        let contains_abusive: Vec<String> = text
            .par_iter()
            .filter_map(|original_str| {
                let filtered_str = self.filter_phrase(original_str.clone());
                self.contains_abusive_words(&filtered_str, &abusive_words)
            })
            .collect();

        if !contains_abusive.is_empty() {
            return Err(domain::services::ServiceError::Censorship(contains_abusive.join(",")));
        }

        Ok(false)
    }

    /// Scans a string for abusive words using fuzzy matching.
    ///
    /// Splits input into words and checks each against the prohibited list.
    /// Returns early if a match is found.
    ///
    /// # Arguments
    /// * `original_str` - Input string to analyze
    /// * `abusive_words` - List of prohibited words to match against
    ///
    /// # Returns
    /// `Some(String)` containing the matched word if found, `None` otherwise
    fn contains_abusive_words(&self, original_str: &str, abusive_words: &[String]) -> Option<String> {
        let words: Vec<&str> = original_str.split_whitespace().collect();

        for word in words {
            if let Some(abusive_word) = self.is_abusive_word_vec(word, abusive_words) {
                return Some(abusive_word);
            }
        }
        None
    }

    /// Performs fuzzy matching of a word against a list of prohibited words.
    ///
    /// Uses Levenshtein distance with configurable sensitivity threshold to
    /// detect variations and misspellings of prohibited words.
    ///
    /// # Arguments
    /// * `word` - Word to check for matches
    /// * `abusive_words` - List of prohibited words to compare against
    ///
    /// # Returns
    /// `Some(String)` with the normalized word if a match is found, `None`
    /// otherwise
    fn is_abusive_word_vec(&self, word: &str, abusive_words: &[String]) -> Option<String> {
        let cleaned_word = word.to_lowercase().replace(' ', "");

        if abusive_words.iter().any(|abusive_word| {
            self.levenshtein_distance(&cleaned_word, abusive_word)
                <= (abusive_word.len() as f32 * self.sensitivity).round() as usize
        }) {
            return Some(cleaned_word);
        }

        None
    }

    /// Normalizes text by replacing common character substitutions.
    ///
    /// # Arguments
    /// * `phrase` - Text to normalize
    ///
    /// # Returns
    /// Normalized text with substituted characters replaced
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

    /// Calculates the Levenshtein distance between two strings.
    ///
    /// # Arguments
    /// * `a` - First string
    /// * `b` - Second string
    ///
    /// # Returns
    /// The Levenshtein distance as a usize
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
                "н": ["н", "n"],
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
        let text = vec!["Не очнеь плоhов".to_string(), "Хя хя нет п".to_string()];

        let result = service.check_abusive_content(&text, true, repo).await;

        assert_eq!(result, Err(domain::services::ServiceError::Censorship("плохов".into())));

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
