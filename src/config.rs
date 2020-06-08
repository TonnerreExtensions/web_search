#[allow(unused_imports)]
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use serde_json::Result;

#[allow(dead_code)]
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

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
        format!(
            "{}/{}",
            self.configurable.main_url.value.trim_end_matches("/"),
            self.internal.query_parameter.replace("{}", &target)
        )
    }

    pub fn suggestion_url(&self, target: &str) -> Option<String> {
        self.internal
            .suggestion_url_template
            .as_ref()
            .map(|internal| internal.replace("{}", target))
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::{Config, ConfigValue, Configurable, Internal};

    #[test]
    fn test_fill_in_template() {
        let config = Config {
            configurable: Configurable {
                main_url: ConfigValue {
                    value: "mainURL".to_owned(),
                },
            },
            internal: Internal {
                query_parameter: "search?q={}".to_owned(),
                suggestion_url_template: Some("suggestion_{}".to_owned()),
            },
        };
        assert_eq!(config.search_url("target"), "mainURL/search?q=target");
        assert_eq!(
            config.suggestion_url("target").unwrap(),
            "suggestion_target"
        );
    }
}
