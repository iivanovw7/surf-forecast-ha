use crate::handlers::meteo::extractors::{ecmwf_wam_at, ewam_at, meteofrance_wave_at};
use crate::handlers::meteo::validators::{validate_marine_hourly, validate_weather_hourly};
use crate::types::forecast::{HourlyForecast, MarineForecast, WindData};
use crate::types::open_meteo::{OpenMeteoMarineResponse, OpenMeteoWeatherResponse};
use anyhow::{Context, Result};

pub async fn get_meteo_data(latitude: f64, longitude: f64) -> Result<MarineForecast> {
    let marine_url = format!(
        "https://marine-api.open-meteo.com/v1/marine\
        ?latitude={latitude}\
        &longitude={longitude}\
        &hourly=wave_height,wave_period,wave_direction,wave_peak_period,\
        wind_wave_height,wind_wave_period,wind_wave_direction,wind_wave_peak_period,\
        swell_wave_height,swell_wave_period,swell_wave_direction,swell_wave_peak_period,\
        secondary_swell_wave_height,secondary_swell_wave_period,secondary_swell_wave_direction,\
        sea_surface_temperature,\
        ocean_current_velocity,ocean_current_direction,\
        sea_level_height_msl\
        &models=ecmwf_wam,ewam,meteofrance_wave\
        &cell_selection=sea\
        &forecast_days=7"
    );

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast\
        ?latitude={latitude}\
        &longitude={longitude}\
        &hourly=wind_speed_10m,wind_direction_10m,wind_gusts_10m\
        &models=ecmwf_ifs,dwd_icon_eu,ecmwf_ifs025"
    );

    let client = reqwest::Client::new();

    let (marine_response, weather_response) = tokio::try_join!(
        client.get(&marine_url).send(),
        client.get(&weather_url).send(),
    )
    .context("Failed to request Open-Meteo APIs")?;

    tracing::debug!(
        marine_response = ?marine_response,
        "Forecast"
    );

    let marine_response = marine_response
        .error_for_status()
        .context("Open-Meteo marine request failed")?;

    let weather_response = weather_response
        .error_for_status()
        .context("Open-Meteo weather request failed")?;

    let marine: OpenMeteoMarineResponse = marine_response
        .json()
        .await
        .context("Failed to deserialize Open-Meteo marine response")?;

    let weather: OpenMeteoWeatherResponse = weather_response
        .json()
        .await
        .context("Failed to deserialize Open-Meteo weather response")?;

    validate_marine_hourly(&marine.hourly).context("Invalid marine hourly data")?;
    validate_weather_hourly(&weather.hourly).context("Invalid weather hourly data")?;

    let hourly = marine
        .hourly
        .time
        .iter()
        .enumerate()
        .map(|(i, time)| HourlyForecast {
            time: time.clone(),

            ecmwf_wam: ecmwf_wam_at(&marine.hourly, i),
            ewam: ewam_at(&marine.hourly, i),
            meteofrance_wave: meteofrance_wave_at(&marine.hourly, i),

            wind_ecmwf_ifs: WindData {
                speed_10m: weather.hourly.wind_speed_10m_ecmwf_ifs[i],
                direction_10m: weather.hourly.wind_direction_10m_ecmwf_ifs[i],
                gusts_10m: weather.hourly.wind_gusts_10m_ecmwf_ifs[i],
            },

            wind_dwd_icon_eu: WindData {
                speed_10m: weather.hourly.wind_speed_10m_dwd_icon_eu[i],
                direction_10m: weather.hourly.wind_direction_10m_dwd_icon_eu[i],
                gusts_10m: weather.hourly.wind_gusts_10m_dwd_icon_eu[i],
            },

            wind_ecmwf_ifs025: WindData {
                speed_10m: weather.hourly.wind_speed_10m_ecmwf_ifs025[i],
                direction_10m: weather.hourly.wind_direction_10m_ecmwf_ifs025[i],
                gusts_10m: weather.hourly.wind_gusts_10m_ecmwf_ifs025[i],
            },
        })
        .collect();

    Ok(MarineForecast {
        latitude: marine.latitude,
        longitude: marine.longitude,
        elevation: marine.elevation,

        timezone: marine.timezone,
        timezone_abbreviation: marine.timezone_abbreviation,
        utc_offset_seconds: marine.utc_offset_seconds,

        generated_at_ms: marine.generationtime_ms,

        hourly,
    })
}
