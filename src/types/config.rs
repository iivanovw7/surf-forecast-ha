use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tera::Tera;
use tokio::sync::Mutex;

use crate::types::forecast::ForecastModel;

pub const OPTIONS_PATH: &str = "/data/options.json";
pub const CONFIG_PATH: &str = "config.yaml";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CssManifest {
    #[serde(flatten)]
    pub entries: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Spot {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub cover: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
}

impl Spot {
    pub fn normalized_cover(&self) -> Option<String> {
        self.cover.as_ref().map(|c| {
            if c.starts_with("/assets/") {
                c.trim_start_matches('/').to_string()
            } else {
                c.clone()
            }
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AddonOptions {
    pub spots: Vec<Spot>,
    #[serde(default)]
    pub model: ForecastModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub options: AddonOptions,
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
    pub forecast_queue: Arc<Mutex<Option<std::time::Instant>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_options() {
        let data = r#"
        {
            "spots": [
                {
                    "id": "sao_joao",
                    "name": "São João da Caparica",
                    "latitude": 38.6517,
                    "longitude": -9.2431
                }
            ]
        }
        "#;

        let options: AddonOptions = serde_json::from_str(data).unwrap();

        assert_eq!(options.spots.len(), 1);
        assert_eq!(options.spots[0].id, "sao_joao");
        assert_eq!(options.spots[0].name, "São João da Caparica");
        assert_eq!(options.spots[0].latitude, 38.6517);
        assert_eq!(options.spots[0].longitude, -9.2431);
    }

    #[test]
    fn test_deserialize_yaml() {
        let yaml_data = r#"
        name: "Surf Forecast"
        options:
          spots:
            - id: "sao_joao"
              name: "São João da Caparica"
              latitude: 38.6517
              longitude: -9.2431
        "#;

        let config_yaml: Config = serde_yaml::from_str(yaml_data).unwrap();

        assert_eq!(config_yaml.options.spots.len(), 1);
        assert_eq!(config_yaml.options.spots[0].id, "sao_joao");
        assert_eq!(config_yaml.options.spots[0].name, "São João da Caparica");
        assert_eq!(config_yaml.options.spots[0].latitude, 38.6517);
        assert_eq!(config_yaml.options.spots[0].longitude, -9.2431);
    }

    #[test]
    fn test_normalized_cover() {
        let spot_with_leading_slash = Spot {
            cover: Some("/assets/jpg/sao-joao.jpg".to_string()),
            ..Default::default()
        };
        assert_eq!(
            spot_with_leading_slash.normalized_cover(),
            Some("assets/jpg/sao-joao.jpg".to_string())
        );

        let spot_relative = Spot {
            cover: Some("assets/jpg/sao-joao.jpg".to_string()),
            ..Default::default()
        };
        assert_eq!(
            spot_relative.normalized_cover(),
            Some("assets/jpg/sao-joao.jpg".to_string())
        );

        let spot_remote = Spot {
            cover: Some("https://example.com/cover.jpg".to_string()),
            ..Default::default()
        };
        assert_eq!(
            spot_remote.normalized_cover(),
            Some("https://example.com/cover.jpg".to_string())
        );

        let spot_none = Spot {
            cover: None,
            ..Default::default()
        };
        assert_eq!(spot_none.normalized_cover(), None);
    }
}
