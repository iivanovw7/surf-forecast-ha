use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarineForecast {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i32,
    pub generated_at_ms: f64,
    pub hourly: Vec<HourlyForecast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotForecast {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub forecast: Option<ForecastModelData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastModelData {
    pub elevation: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i32,
    pub generated_at_ms: f64,
    pub hourly: HourlyForecastData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HourlyForecastData {
    #[serde(default)]
    pub time: Vec<String>,
    #[serde(default)]
    pub wave_height: Vec<Option<f64>>,
    #[serde(default)]
    pub swell_wave_height: Vec<Option<f64>>,
    #[serde(default)]
    pub swell_wave_direction: Vec<Option<f64>>,
    #[serde(default)]
    pub swell_wave_period: Vec<Option<f64>>,
    #[serde(default)]
    pub wind_speed: Vec<Option<f64>>,
    #[serde(default)]
    pub wind_direction: Vec<Option<f64>>,
    #[serde(default)]
    pub wind_gust: Vec<Option<f64>>,
    #[serde(default)]
    pub water_temperature: Vec<Option<f64>>,
    #[serde(default)]
    pub air_temperature: Vec<Option<f64>>,
}

impl MarineForecast {
    pub fn selected_hourly(&self, model: ForecastModel) -> Vec<HourlyModelForecast> {
        self.hourly
            .iter()
            .map(|hour| {
                let (wave, wind) = hour.model_data(model);

                HourlyModelForecast {
                    time: hour.time.clone(),
                    model,
                    wave: wave.clone(),
                    wind: wind.clone(),
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ForecastModel {
    #[default]
    #[serde(alias = "ecmwf_ifs")]
    EcmwfWam,

    #[serde(alias = "dwd_icon_eu", alias = "icon_eu")]
    Ewam,

    #[serde(alias = "ecmwf_ifs025", alias = "ecmwf_ifs_025", alias = "meteofrance")]
    MeteofranceWave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyModelForecast {
    pub time: String,
    pub model: ForecastModel,
    pub wave: WaveModelData,
    pub wind: WindData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub time: String,

    pub ecmwf_wam: WaveModelData,
    pub ewam: WaveModelData,
    pub meteofrance_wave: WaveModelData,

    pub wind_ecmwf_ifs: WindData,
    pub wind_dwd_icon_eu: WindData,
    pub wind_ecmwf_ifs025: WindData,
}

impl HourlyForecast {
    pub fn model_data(&self, model: ForecastModel) -> (&WaveModelData, &WindData) {
        match model {
            ForecastModel::EcmwfWam => (&self.ecmwf_wam, &self.wind_ecmwf_ifs),
            ForecastModel::Ewam => (&self.ewam, &self.wind_dwd_icon_eu),
            ForecastModel::MeteofranceWave => (&self.meteofrance_wave, &self.wind_ecmwf_ifs025),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveModelData {
    pub wave: WaveData,
    pub wind_wave: WaveData,
    pub swell: SwellData,
    pub secondary_swell: SwellData,

    pub sea_surface_temperature: Option<f64>,
    pub ocean_current_velocity: Option<f64>,
    pub ocean_current_direction: Option<f64>,
    pub sea_level_height_msl: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveData {
    pub height: Option<f64>,
    pub period: Option<f64>,
    pub direction: Option<f64>,
    pub peak_period: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwellData {
    pub height: Option<f64>,
    pub period: Option<f64>,
    pub direction: Option<f64>,
    pub peak_period: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindData {
    pub speed_10m: Option<f64>,
    pub direction_10m: Option<f64>,
    pub gusts_10m: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpotView {
    pub id: String,
    pub name: String,
    pub location: String,
    pub cover: String,
    pub has_data: bool,
    pub days: Vec<SpotForecastDayView>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DayHeaderView {
    pub day_name: String,
    pub date_str: String,
    pub is_today: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpotForecastDayView {
    pub day_name: String,
    pub date_str: String,
    pub is_today: bool,
    pub has_data: bool,
    pub rating: String,
    pub rating_class: String,
    pub condition_tag: Option<String>,
    pub wave_height_m: String,
    pub swell_direction: String,
    pub swell_deg: f64,
    pub swell_period: String,
    pub water_temp: String,
    pub wind_speed: String,
    pub wind_gust: String,
    pub wind_direction: String,
    pub wind_deg: f64,
    pub air_temp: String,
}

impl SpotForecastDayView {
    pub fn empty(header: &DayHeaderView) -> Self {
        Self {
            day_name: header.day_name.clone(),
            date_str: header.date_str.clone(),
            is_today: header.is_today,
            has_data: false,
            rating: "-".to_string(),
            rating_class: "empty".to_string(),
            condition_tag: None,
            wave_height_m: "-".to_string(),
            swell_direction: "-".to_string(),
            swell_deg: 0.0,
            swell_period: "-".to_string(),
            water_temp: "-".to_string(),
            wind_speed: "-".to_string(),
            wind_gust: "-".to_string(),
            wind_direction: "-".to_string(),
            wind_deg: 0.0,
            air_temp: "-".to_string(),
        }
    }
}

pub struct Rating {
    pub label: &'static str,
    pub class_name: &'static str,
    pub tag: Option<&'static str>,
}
