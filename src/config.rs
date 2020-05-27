use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
pub struct Config {
    configurable: Configurable,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configurable {
    search_url_template: ConfigValue,
    suggestion_url_template: Option<ConfigValue>,
}

#[derive(Deserialize)]
struct ConfigValue {
    value: String,
}

impl Config {
    pub fn from(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }

    pub fn search_url(&self, target: &str) -> String {
        self.configurable
            .search_url_template
            .value
            .replace("{}", target)
    }

    pub fn suggestion_url(&self, target: &str) -> Option<String> {
        self.configurable
            .suggestion_url_template
            .as_ref()
            .map(|suggestion| suggestion.value.replace("{}", target))
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::{Config, ConfigValue, Configurable};

    #[test]
    fn test_fill_in_template() {
        let config = Config {
            configurable: Configurable {
                search_url_template: ConfigValue {
                    value: "search_{}".to_owned(),
                },
                suggestion_url_template: Some(ConfigValue {
                    value: "suggestion_{}".to_owned(),
                }),
            },
        };
        assert_eq!(config.search_url("target"), "search_target");
        assert_eq!(
            config.suggestion_url("target").unwrap(),
            "suggestion_target"
        );
    }
}
