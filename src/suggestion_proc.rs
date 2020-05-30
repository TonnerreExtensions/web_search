use crate::config::Config;
use crate::service::Service;
use htmlescape::decode_html;
#[allow(unused_imports)]
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde_json::Value;

#[allow(dead_code)]
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'&');

#[cfg(feature = "google")]
fn process_json(json: &Value, config: &Config) -> Option<Vec<Service>> {
    let json = json
        .as_array()?
        .get(1)?
        .as_array()?
        .iter()
        .filter_map(|suggestion| suggestion.get(0)?.as_str())
        .map(decode_html)
        .filter_map(Result::ok)
        .map(|suggestion| {
            let search_url =
                config.search_url(&utf8_percent_encode(&suggestion, FRAGMENT).to_string());
            crate::query::build_service(search_url, &suggestion, &suggestion)
        })
        .collect::<Vec<_>>();
    Some(json)
}

#[cfg(feature = "wikipedia")]
fn process_json(json: &Value, _config: &Config) -> Option<Vec<Service>> {
    let titles = json
        .as_array()?
        .get(1)?
        .as_array()?
        .into_iter()
        .map(|val| val.as_str());
    let urls = json
        .as_array()?
        .get(3)?
        .as_array()?
        .into_iter()
        .map(|val| val.as_str());
    let services = titles
        .zip(urls)
        .filter_map(|(title, url)| Some((decode_html(title?).ok()?, url?)))
        .map(|(title, url)| crate::query::build_service(url, &title, &title))
        .collect();
    Some(services)
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
    process_json(&response, config).unwrap_or_default()
}
