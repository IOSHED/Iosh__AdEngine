lazy_static::lazy_static! {
    pub static ref RE_GENDER: regex::Regex = regex::Regex::new(r"^(MALE|FEMALE)$").unwrap();
}
