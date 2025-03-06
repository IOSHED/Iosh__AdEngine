lazy_static::lazy_static! {
    pub static ref RE_GENDER: regex::Regex = regex::Regex::new(r"^(MALE|FEMALE)$").unwrap();
    pub static ref RE_GENERATE_TYPE: regex::Regex = regex::Regex::new(r"^(TEXT|TITLE|ALL)$").unwrap();
}
