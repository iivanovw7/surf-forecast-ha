use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoMarineResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,

    pub hourly_units: OpenMeteoMarineHourlyUnits,
    pub hourly: OpenMeteoMarineHourly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoMarineHourlyUnits {
    pub time: String,

    // ECMWF WAM
    #[serde(default)]
    pub wave_height_ecmwf_wam: String,
    #[serde(default)]
    pub wave_period_ecmwf_wam: String,
    #[serde(default)]
    pub wave_direction_ecmwf_wam: String,
    #[serde(default)]
    pub wave_peak_period_ecmwf_wam: String,

    #[serde(default)]
    pub wind_wave_height_ecmwf_wam: String,
    #[serde(default)]
    pub wind_wave_period_ecmwf_wam: String,
    #[serde(default)]
    pub wind_wave_direction_ecmwf_wam: String,
    #[serde(default)]
    pub wind_wave_peak_period_ecmwf_wam: String,

    #[serde(default)]
    pub swell_wave_height_ecmwf_wam: String,
    #[serde(default)]
    pub swell_wave_period_ecmwf_wam: String,
    #[serde(default)]
    pub swell_wave_direction_ecmwf_wam: String,
    #[serde(default)]
    pub swell_wave_peak_period_ecmwf_wam: String,

    #[serde(default)]
    pub secondary_swell_wave_height_ecmwf_wam: String,
    #[serde(default)]
    pub secondary_swell_wave_period_ecmwf_wam: String,
    #[serde(default)]
    pub secondary_swell_wave_direction_ecmwf_wam: String,

    #[serde(default)]
    pub sea_surface_temperature_ecmwf_wam: String,
    #[serde(default)]
    pub ocean_current_velocity_ecmwf_wam: String,
    #[serde(default)]
    pub ocean_current_direction_ecmwf_wam: String,

    #[serde(default)]
    pub sea_level_height_msl_ecmwf_wam: String,

    // EWAM
    #[serde(default)]
    pub wave_height_ewam: String,
    #[serde(default)]
    pub wave_period_ewam: String,
    #[serde(default)]
    pub wave_direction_ewam: String,
    #[serde(default)]
    pub wave_peak_period_ewam: String,

    #[serde(default)]
    pub wind_wave_height_ewam: String,
    #[serde(default)]
    pub wind_wave_period_ewam: String,
    #[serde(default)]
    pub wind_wave_direction_ewam: String,
    #[serde(default)]
    pub wind_wave_peak_period_ewam: String,

    #[serde(default)]
    pub swell_wave_height_ewam: String,
    #[serde(default)]
    pub swell_wave_period_ewam: String,
    #[serde(default)]
    pub swell_wave_direction_ewam: String,
    #[serde(default)]
    pub swell_wave_peak_period_ewam: String,

    #[serde(default)]
    pub secondary_swell_wave_height_ewam: String,
    #[serde(default)]
    pub secondary_swell_wave_period_ewam: String,
    #[serde(default)]
    pub secondary_swell_wave_direction_ewam: String,

    #[serde(default)]
    pub sea_surface_temperature_ewam: String,
    #[serde(default)]
    pub ocean_current_velocity_ewam: String,
    #[serde(default)]
    pub ocean_current_direction_ewam: String,

    #[serde(default)]
    pub sea_level_height_msl_ewam: String,

    // METEOFRANCE WAVE
    #[serde(default)]
    pub wave_height_meteofrance_wave: String,
    #[serde(default)]
    pub wave_period_meteofrance_wave: String,
    #[serde(default)]
    pub wave_direction_meteofrance_wave: String,
    #[serde(default)]
    pub wave_peak_period_meteofrance_wave: String,

    #[serde(default)]
    pub wind_wave_height_meteofrance_wave: String,
    #[serde(default)]
    pub wind_wave_period_meteofrance_wave: String,
    #[serde(default)]
    pub wind_wave_direction_meteofrance_wave: String,
    #[serde(default)]
    pub wind_wave_peak_period_meteofrance_wave: String,

    #[serde(default)]
    pub swell_wave_height_meteofrance_wave: String,
    #[serde(default)]
    pub swell_wave_period_meteofrance_wave: String,
    #[serde(default)]
    pub swell_wave_direction_meteofrance_wave: String,
    #[serde(default)]
    pub swell_wave_peak_period_meteofrance_wave: String,

    #[serde(default)]
    pub secondary_swell_wave_height_meteofrance_wave: String,
    #[serde(default)]
    pub secondary_swell_wave_period_meteofrance_wave: String,
    #[serde(default)]
    pub secondary_swell_wave_direction_meteofrance_wave: String,

    #[serde(default)]
    pub sea_surface_temperature_meteofrance_wave: String,
    #[serde(default)]
    pub ocean_current_velocity_meteofrance_wave: String,
    #[serde(default)]
    pub ocean_current_direction_meteofrance_wave: String,

    #[serde(default)]
    pub sea_level_height_msl_meteofrance_wave: String,
}

pub type NullableF64 = Option<f64>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoMarineHourly {
    pub time: Vec<String>,

    // ECMWF WAM
    #[serde(default)]
    pub wave_height_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_period_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_direction_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_peak_period_ecmwf_wam: Vec<NullableF64>,

    #[serde(default)]
    pub wind_wave_height_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_period_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_direction_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_peak_period_ecmwf_wam: Vec<NullableF64>,

    #[serde(default)]
    pub swell_wave_height_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_period_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_direction_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_peak_period_ecmwf_wam: Vec<NullableF64>,

    #[serde(default)]
    pub secondary_swell_wave_height_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_period_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_direction_ecmwf_wam: Vec<NullableF64>,

    #[serde(default)]
    pub sea_surface_temperature_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_velocity_ecmwf_wam: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_direction_ecmwf_wam: Vec<NullableF64>,

    #[serde(default)]
    pub sea_level_height_msl_ecmwf_wam: Vec<NullableF64>,

    // EWAM
    #[serde(default)]
    pub wave_height_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_period_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_direction_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wave_peak_period_ewam: Vec<NullableF64>,

    #[serde(default)]
    pub wind_wave_height_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_period_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_direction_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_peak_period_ewam: Vec<NullableF64>,

    #[serde(default)]
    pub swell_wave_height_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_period_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_direction_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_peak_period_ewam: Vec<NullableF64>,

    #[serde(default)]
    pub secondary_swell_wave_height_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_period_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_direction_ewam: Vec<NullableF64>,

    #[serde(default)]
    pub sea_surface_temperature_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_velocity_ewam: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_direction_ewam: Vec<NullableF64>,

    #[serde(default)]
    pub sea_level_height_msl_ewam: Vec<NullableF64>,

    // METEOFRANCE WAVE
    #[serde(default)]
    pub wave_height_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wave_period_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wave_direction_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wave_peak_period_meteofrance_wave: Vec<NullableF64>,

    #[serde(default)]
    pub wind_wave_height_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_period_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_direction_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub wind_wave_peak_period_meteofrance_wave: Vec<NullableF64>,

    #[serde(default)]
    pub swell_wave_height_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_period_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_direction_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub swell_wave_peak_period_meteofrance_wave: Vec<NullableF64>,

    #[serde(default)]
    pub secondary_swell_wave_height_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_period_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub secondary_swell_wave_direction_meteofrance_wave: Vec<NullableF64>,

    #[serde(default)]
    pub sea_surface_temperature_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_velocity_meteofrance_wave: Vec<NullableF64>,
    #[serde(default)]
    pub ocean_current_direction_meteofrance_wave: Vec<NullableF64>,

    #[serde(default)]
    pub sea_level_height_msl_meteofrance_wave: Vec<NullableF64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoWeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,

    pub hourly_units: OpenMeteoWeatherHourlyUnits,
    pub hourly: OpenMeteoWeatherHourly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoWeatherHourlyUnits {
    pub time: String,

    // ECMWF IFS
    pub wind_speed_10m_ecmwf_ifs: String,
    pub wind_direction_10m_ecmwf_ifs: String,
    pub wind_gusts_10m_ecmwf_ifs: String,

    // DWD ICON EU
    pub wind_speed_10m_dwd_icon_eu: String,
    pub wind_direction_10m_dwd_icon_eu: String,
    pub wind_gusts_10m_dwd_icon_eu: String,

    // ECMWF IFS 0.25
    pub wind_speed_10m_ecmwf_ifs025: String,
    pub wind_direction_10m_ecmwf_ifs025: String,
    pub wind_gusts_10m_ecmwf_ifs025: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMeteoWeatherHourly {
    pub time: Vec<String>,

    // ECMWF IFS
    pub wind_speed_10m_ecmwf_ifs: Vec<Option<f64>>,
    pub wind_direction_10m_ecmwf_ifs: Vec<Option<f64>>,
    pub wind_gusts_10m_ecmwf_ifs: Vec<Option<f64>>,

    // DWD ICON EU
    pub wind_speed_10m_dwd_icon_eu: Vec<Option<f64>>,
    pub wind_direction_10m_dwd_icon_eu: Vec<Option<f64>>,
    pub wind_gusts_10m_dwd_icon_eu: Vec<Option<f64>>,

    // ECMWF IFS 0.25
    pub wind_speed_10m_ecmwf_ifs025: Vec<Option<f64>>,
    pub wind_direction_10m_ecmwf_ifs025: Vec<Option<f64>>,
    pub wind_gusts_10m_ecmwf_ifs025: Vec<Option<f64>>,
}
