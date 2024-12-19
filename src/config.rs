use crate::error::Error;
use regex::Captures;
use regex::Regex;

use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    asana_base_url: String,
    db_url: String,
}

pub async fn config() -> Result<AppConfig, Error> {
    let asana_base_url = env::var("ASANA_BASE_URL").expect("ASANA_BASE_URL is missing!");

    Ok(AppConfig {
        asana_base_url,
        db_url: String::from(""),
    })
}

pub fn sanitize_db_url(url: &str) -> Result<String, Error> {
    let re = Regex::new(r"^(postgres://[a-zA-Z\d\-\S]+):([a-zA-Z\d\-\S]*)@")?;
    let result = re
        .replace(url, |caps: &Captures<'_>| {
            format!("{}:<PASSWORD_REDACTED>@", &caps[1])
        })
        .to_string();

    Ok(result)
}
