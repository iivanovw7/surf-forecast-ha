use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Tera;

pub const OPTIONS_PATH: &str = "/data/options.json";
pub const CONFIG_PATH: &str = "config.yaml";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Spot {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AddonOptions {
    pub spots: Vec<Spot>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub options: AddonOptions,
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
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
}
