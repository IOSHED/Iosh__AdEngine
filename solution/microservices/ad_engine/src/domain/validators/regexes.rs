lazy_static::lazy_static! {
    pub static ref RE_DATE: regex::Regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    pub static ref RE_ALPHA2: regex::Regex = regex::Regex::new(r"[a-zA-Z]{2}").unwrap();
}
