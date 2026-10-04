use crate::types::{
    forecast::{SwellData, WaveData, WaveModelData},
    open_meteo::OpenMeteoMarineHourly,
};

pub fn ecmwf_wam_at(hourly: &OpenMeteoMarineHourly, i: usize) -> WaveModelData {
    WaveModelData {
        wave: WaveData {
            height: hourly.wave_height_ecmwf_wam.get(i).copied().flatten(),
            period: hourly.wave_period_ecmwf_wam.get(i).copied().flatten(),
            direction: hourly.wave_direction_ecmwf_wam.get(i).copied().flatten(),
            peak_period: hourly.wave_peak_period_ecmwf_wam.get(i).copied().flatten(),
        },

        wind_wave: WaveData {
            height: hourly.wind_wave_height_ecmwf_wam.get(i).copied().flatten(),
            period: hourly.wind_wave_period_ecmwf_wam.get(i).copied().flatten(),
            direction: hourly
                .wind_wave_direction_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
            peak_period: hourly
                .wind_wave_peak_period_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
        },

        swell: SwellData {
            height: hourly.swell_wave_height_ecmwf_wam.get(i).copied().flatten(),
            period: hourly.swell_wave_period_ecmwf_wam.get(i).copied().flatten(),
            direction: hourly
                .swell_wave_direction_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
            peak_period: hourly
                .swell_wave_peak_period_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
        },

        secondary_swell: SwellData {
            height: hourly
                .secondary_swell_wave_height_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .secondary_swell_wave_period_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .secondary_swell_wave_direction_ecmwf_wam
                .get(i)
                .copied()
                .flatten(),
            peak_period: None,
        },

        sea_surface_temperature: hourly
            .sea_surface_temperature_ecmwf_wam
            .get(i)
            .copied()
            .flatten(),
        ocean_current_velocity: hourly
            .ocean_current_velocity_ecmwf_wam
            .get(i)
            .copied()
            .flatten(),
        ocean_current_direction: hourly
            .ocean_current_direction_ecmwf_wam
            .get(i)
            .copied()
            .flatten(),
        sea_level_height_msl: hourly
            .sea_level_height_msl_ecmwf_wam
            .get(i)
            .copied()
            .flatten(),
    }
}

pub fn ewam_at(hourly: &OpenMeteoMarineHourly, i: usize) -> WaveModelData {
    WaveModelData {
        wave: WaveData {
            height: hourly.wave_height_ewam.get(i).copied().flatten(),
            period: hourly.wave_period_ewam.get(i).copied().flatten(),
            direction: hourly.wave_direction_ewam.get(i).copied().flatten(),
            peak_period: hourly.wave_peak_period_ewam.get(i).copied().flatten(),
        },

        wind_wave: WaveData {
            height: hourly.wind_wave_height_ewam.get(i).copied().flatten(),
            period: hourly.wind_wave_period_ewam.get(i).copied().flatten(),
            direction: hourly.wind_wave_direction_ewam.get(i).copied().flatten(),
            peak_period: hourly.wind_wave_peak_period_ewam.get(i).copied().flatten(),
        },

        swell: SwellData {
            height: hourly.swell_wave_height_ewam.get(i).copied().flatten(),
            period: hourly.swell_wave_period_ewam.get(i).copied().flatten(),
            direction: hourly.swell_wave_direction_ewam.get(i).copied().flatten(),
            peak_period: hourly.swell_wave_peak_period_ewam.get(i).copied().flatten(),
        },

        secondary_swell: SwellData {
            height: hourly
                .secondary_swell_wave_height_ewam
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .secondary_swell_wave_period_ewam
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .secondary_swell_wave_direction_ewam
                .get(i)
                .copied()
                .flatten(),
            peak_period: None,
        },

        sea_surface_temperature: hourly
            .sea_surface_temperature_ewam
            .get(i)
            .copied()
            .flatten(),
        ocean_current_velocity: hourly.ocean_current_velocity_ewam.get(i).copied().flatten(),
        ocean_current_direction: hourly
            .ocean_current_direction_ewam
            .get(i)
            .copied()
            .flatten(),
        sea_level_height_msl: hourly.sea_level_height_msl_ewam.get(i).copied().flatten(),
    }
}

pub fn meteofrance_wave_at(hourly: &OpenMeteoMarineHourly, i: usize) -> WaveModelData {
    WaveModelData {
        wave: WaveData {
            height: hourly
                .wave_height_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .wave_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .wave_direction_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            peak_period: hourly
                .wave_peak_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
        },

        wind_wave: WaveData {
            height: hourly
                .wind_wave_height_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .wind_wave_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .wind_wave_direction_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            peak_period: hourly
                .wind_wave_peak_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
        },

        swell: SwellData {
            height: hourly
                .swell_wave_height_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .swell_wave_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .swell_wave_direction_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            peak_period: hourly
                .swell_wave_peak_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
        },

        secondary_swell: SwellData {
            height: hourly
                .secondary_swell_wave_height_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            period: hourly
                .secondary_swell_wave_period_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            direction: hourly
                .secondary_swell_wave_direction_meteofrance_wave
                .get(i)
                .copied()
                .flatten(),
            peak_period: None,
        },

        sea_surface_temperature: hourly
            .sea_surface_temperature_meteofrance_wave
            .get(i)
            .copied()
            .flatten(),
        ocean_current_velocity: hourly
            .ocean_current_velocity_meteofrance_wave
            .get(i)
            .copied()
            .flatten(),
        ocean_current_direction: hourly
            .ocean_current_direction_meteofrance_wave
            .get(i)
            .copied()
            .flatten(),
        sea_level_height_msl: hourly
            .sea_level_height_msl_meteofrance_wave
            .get(i)
            .copied()
            .flatten(),
    }
}
