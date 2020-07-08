use crate::config::Config;
use htmlescape::decode_html;
use serde_json::Value;

#[cfg(feature = "google")]
fn process_json(json: &Value, config: &Config) -> Option<()> {
    json.as_array()?
        .get(1)?
        .as_array()?
        .iter()
        .filter_map(|suggestion| suggestion.get(0)?.as_str())
        .map(decode_html)
        .filter_map(Result::ok)
        .map(|suggestion| {
            let search_url = config.search_url(&suggestion);
            crate::query::build_service(&search_url, &suggestion, &suggestion)
        })
        .filter_map(|service| service.serialize_to_json().ok())
        .for_each(|json| println!("{}", json));
    Some(())
}

#[cfg(feature = "wikipedia")]
fn process_json(json: &Value, _config: &Config) -> Option<()> {
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
    titles
        .zip(urls)
        .filter_map(|(title, url)| Some((decode_html(title?).ok()?, url?)))
        .map(|(title, url)| crate::query::build_service(url, &title, &title))
        .filter_map(|service| service.serialize_to_json().ok())
        .for_each(|json| println!("{}", json));
    Some(())
}

#[cfg(feature = "google_maps")]
fn process_json(json: &Value, config: &Config) -> Option<()> {
    use crate::service::Service;
    let predictions = json.get("predictions")?.as_array()?;
    for prediction in predictions {
        let text_structure = prediction.get("structured_formatting")?;
        let title = decode_html(text_structure.get("main_text")?.as_str()?).ok()?;
        let subtitle = decode_html(text_structure.get("secondary_text")?.as_str()?).ok()?;
        let url = config.search_url(prediction.get("description")?.as_str()?);
        if let Ok(json) = (Service {
            id: url,
            title,
            subtitle,
        })
        .serialize_to_json()
        {
            println!("{}", json)
        }
    }
    Some(())
}

pub fn process_suggestions(config: &Config, request: &str) {
    let suggestion_url = match config.suggestion_url(request) {
        Some(url) => url,
        None => return,
    };
    let response = match ureq::get(&suggestion_url).call().into_json() {
        Ok(response) => response,
        Err(error) => {
            eprint!(env!("PROVIDER_NAME"));
            eprintln!(": Request error: {}", error);
            return;
        }
    };
    process_json(&response, config).unwrap_or_default()
}
