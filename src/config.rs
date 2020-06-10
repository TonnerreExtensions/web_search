#[allow(unused_imports)]
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use serde_json::Result;

#[allow(dead_code)]
const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'&');

#[derive(Deserialize)]
pub struct Config {
    configurable: Configurable,
    internal: Internal,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Internal {
    query_parameter: String,
    suggestion_url_template: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configurable {
    main_url: ConfigValue,
    #[cfg(feature = "api_key")]
    api_key: ConfigValue,
}

#[derive(Deserialize)]
struct ConfigValue {
    value: String,
}

impl Config {
    pub fn from(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }

    pub fn main_url(&self) -> &str {
        self.configurable.main_url.value.as_str()
    }

    pub fn search_url(&self, target: &str) -> String {
        let target = utf8_percent_encode(target, FRAGMENT).to_string();
        let main_url = self.configurable.main_url.value.trim_end_matches("/");
        #[cfg(feature = "api_key")]
        let main_url = main_url.replace("{API_KEY}", &self.configurable.api_key.value);
        format!(
            "{}/{}",
            main_url,
            self.internal.query_parameter.replace("{}", &target)
        )
    }

    pub fn suggestion_url(&self, target: &str) -> Option<String> {
        let suggestion_url = self
            .internal
            .suggestion_url_template
            .as_ref()
            .map(|internal| internal.replace("{}", target));
        #[cfg(feature = "api_key")]
        let suggestion_url = suggestion_url
            .map(|internal| internal.replace("{API_KEY}", &self.configurable.api_key.value));

        suggestion_url
    }
}
