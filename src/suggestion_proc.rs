use crate::config::Config;
use crate::service::Service;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde_json::Value;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'&');

#[cfg(feature = "google")]
fn process_json(json: &Value) -> Option<Vec<&str>> {
    let json = json
        .as_array()?
        .get(1)?
        .as_array()?
        .iter()
        .filter_map(|suggestion| suggestion.get(0)?.as_str())
        .collect::<Vec<_>>();
    Some(json)
}

pub fn process_suggestions(config: &Config, request: &str) -> Vec<Service> {
    let suggestion_url = match config.suggestion_url(request) {
        Some(url) => url,
        None => return vec![],
    };
    let response = match ureq::get(&suggestion_url).call().into_json() {
        Ok(response) => response,
        Err(error) => {
            println!("Request error: {}", error);
            return vec![];
        }
    };
    let suggestions = process_json(&response).unwrap_or_default();
    suggestions
        .into_iter()
        .map(|suggestion| {
            let search_url =
                config.search_url(&utf8_percent_encode(&suggestion, FRAGMENT).to_string());
            crate::query::build_service(search_url, suggestion, suggestion)
        })
        .collect()
}
