pub fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<tracing::level_filters::LevelFilter, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = serde::Deserialize::deserialize(deserializer)?;
    match string.to_lowercase().as_str() {
        "off" => Ok(tracing::level_filters::LevelFilter::OFF),
        "error" => Ok(tracing::level_filters::LevelFilter::ERROR),
        "warn" => Ok(tracing::level_filters::LevelFilter::WARN),
        "info" => Ok(tracing::level_filters::LevelFilter::INFO),
        "debug" => Ok(tracing::level_filters::LevelFilter::DEBUG),
        "trace" => Ok(tracing::level_filters::LevelFilter::TRACE),
        _ => Err(serde::de::Error::custom(format!("Invalid level filter: {}", string))),
    }
}
