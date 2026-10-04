use crate::types::open_meteo::{OpenMeteoMarineHourly, OpenMeteoWeatherHourly};

pub fn validate_weather_hourly(hourly: &OpenMeteoWeatherHourly) -> anyhow::Result<()> {
    let expected = hourly.time.len();

    if hourly.wind_speed_10m_ecmwf_ifs.len() != expected {
        anyhow::bail!(
            "wind_speed_10m_ecmwf_ifs has {} values, expected {}",
            hourly.wind_speed_10m_ecmwf_ifs.len(),
            expected
        );
    }

    if hourly.wind_direction_10m_ecmwf_ifs.len() != expected {
        anyhow::bail!(
            "wind_direction_10m_ecmwf_ifs has {} values, expected {}",
            hourly.wind_direction_10m_ecmwf_ifs.len(),
            expected
        );
    }

    if hourly.wind_gusts_10m_ecmwf_ifs.len() != expected {
        anyhow::bail!(
            "wind_gusts_10m_ecmwf_ifs has {} values, expected {}",
            hourly.wind_gusts_10m_ecmwf_ifs.len(),
            expected
        );
    }

    if hourly.wind_speed_10m_dwd_icon_eu.len() != expected {
        anyhow::bail!(
            "wind_speed_10m_dwd_icon_eu has {} values, expected {}",
            hourly.wind_speed_10m_dwd_icon_eu.len(),
            expected
        );
    }

    if hourly.wind_direction_10m_dwd_icon_eu.len() != expected {
        anyhow::bail!(
            "wind_direction_10m_dwd_icon_eu has {} values, expected {}",
            hourly.wind_direction_10m_dwd_icon_eu.len(),
            expected
        );
    }

    if hourly.wind_gusts_10m_dwd_icon_eu.len() != expected {
        anyhow::bail!(
            "wind_gusts_10m_dwd_icon_eu has {} values, expected {}",
            hourly.wind_gusts_10m_dwd_icon_eu.len(),
            expected
        );
    }

    if hourly.wind_speed_10m_ecmwf_ifs025.len() != expected {
        anyhow::bail!(
            "wind_speed_10m_ecmwf_ifs025 has {} values, expected {}",
            hourly.wind_speed_10m_ecmwf_ifs025.len(),
            expected
        );
    }

    if hourly.wind_direction_10m_ecmwf_ifs025.len() != expected {
        anyhow::bail!(
            "wind_direction_10m_ecmwf_ifs025 has {} values, expected {}",
            hourly.wind_direction_10m_ecmwf_ifs025.len(),
            expected
        );
    }

    if hourly.wind_gusts_10m_ecmwf_ifs025.len() != expected {
        anyhow::bail!(
            "wind_gusts_10m_ecmwf_ifs025 has {} values, expected {}",
            hourly.wind_gusts_10m_ecmwf_ifs025.len(),
            expected
        );
    }

    Ok(())
}

pub fn validate_marine_hourly(hourly: &OpenMeteoMarineHourly) -> anyhow::Result<()> {
    let expected = hourly.time.len();

    macro_rules! check {
        ($($field:ident),+ $(,)?) => {
            $(
                if !hourly.$field.is_empty() && hourly.$field.len() != expected {
                    anyhow::bail!(
                        "Marine field '{}' has {} values, expected {}",
                        stringify!($field),
                        hourly.$field.len(),
                        expected
                    );
                }
            )+
        };
    }

    check!(
        // ECMWF WAM
        wave_height_ecmwf_wam,
        wave_period_ecmwf_wam,
        wave_direction_ecmwf_wam,
        wave_peak_period_ecmwf_wam,
        wind_wave_height_ecmwf_wam,
        wind_wave_period_ecmwf_wam,
        wind_wave_direction_ecmwf_wam,
        wind_wave_peak_period_ecmwf_wam,
        swell_wave_height_ecmwf_wam,
        swell_wave_period_ecmwf_wam,
        swell_wave_direction_ecmwf_wam,
        swell_wave_peak_period_ecmwf_wam,
        secondary_swell_wave_height_ecmwf_wam,
        secondary_swell_wave_period_ecmwf_wam,
        secondary_swell_wave_direction_ecmwf_wam,
        sea_surface_temperature_ecmwf_wam,
        ocean_current_velocity_ecmwf_wam,
        ocean_current_direction_ecmwf_wam,
        sea_level_height_msl_ecmwf_wam,
        // EWAM
        wave_height_ewam,
        wave_period_ewam,
        wave_direction_ewam,
        wave_peak_period_ewam,
        wind_wave_height_ewam,
        wind_wave_period_ewam,
        wind_wave_direction_ewam,
        wind_wave_peak_period_ewam,
        swell_wave_height_ewam,
        swell_wave_period_ewam,
        swell_wave_direction_ewam,
        swell_wave_peak_period_ewam,
        secondary_swell_wave_height_ewam,
        secondary_swell_wave_period_ewam,
        secondary_swell_wave_direction_ewam,
        sea_surface_temperature_ewam,
        ocean_current_velocity_ewam,
        ocean_current_direction_ewam,
        sea_level_height_msl_ewam,
        // METEOFRANCE WAVE
        wave_height_meteofrance_wave,
        wave_period_meteofrance_wave,
        wave_direction_meteofrance_wave,
        wave_peak_period_meteofrance_wave,
        wind_wave_height_meteofrance_wave,
        wind_wave_period_meteofrance_wave,
        wind_wave_direction_meteofrance_wave,
        wind_wave_peak_period_meteofrance_wave,
        swell_wave_height_meteofrance_wave,
        swell_wave_period_meteofrance_wave,
        swell_wave_direction_meteofrance_wave,
        swell_wave_peak_period_meteofrance_wave,
        secondary_swell_wave_height_meteofrance_wave,
        secondary_swell_wave_period_meteofrance_wave,
        secondary_swell_wave_direction_meteofrance_wave,
        sea_surface_temperature_meteofrance_wave,
        ocean_current_velocity_meteofrance_wave,
        ocean_current_direction_meteofrance_wave,
        sea_level_height_msl_meteofrance_wave,
    );

    Ok(())
}
