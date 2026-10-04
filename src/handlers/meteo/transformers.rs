use chrono::{NaiveDate, Utc};

use crate::types::forecast::{
    DayHeaderView, ForecastModel, MarineForecast, Rating, SpotForecastDayView, WaveModelData,
    WindData,
};

pub fn build_daily_forecast(
    forecast: &MarineForecast,
    model: ForecastModel,
) -> Vec<SpotForecastDayView> {
    if forecast.hourly.is_empty() {
        return Vec::new();
    }

    let today = forecast
        .hourly
        .first()
        .and_then(|hour| parse_forecast_date(&hour.time))
        .unwrap_or_else(|| Utc::now().date_naive());

    let day_headers = build_day_headers(today);

    let mut result = Vec::with_capacity(7);

    for day_index in 0..7 {
        let Some(header) = day_headers.get(day_index) else {
            break;
        };

        let target_date = today + chrono::Duration::days(day_index as i64);

        let day_data: Vec<(&WaveModelData, &WindData)> = forecast
            .hourly
            .iter()
            .filter(|hour| parse_forecast_date(&hour.time) == Some(target_date))
            .map(|hour| hour.model_data(model))
            .collect();

        if day_data.is_empty() {
            result.push(SpotForecastDayView::empty(header));
            continue;
        }

        let wave_heights: Vec<f64> = day_data
            .iter()
            .filter_map(|(wave, _)| wave.wave.height)
            .collect();

        let swell_heights: Vec<f64> = day_data
            .iter()
            .filter_map(|(wave, _)| wave.swell.height)
            .collect();

        let swell_directions: Vec<f64> = day_data
            .iter()
            .filter_map(|(wave, _)| wave.swell.direction)
            .collect();

        let swell_periods: Vec<f64> = day_data
            .iter()
            .filter_map(|(wave, _)| wave.swell.period)
            .collect();

        let wind_speeds: Vec<f64> = day_data
            .iter()
            .filter_map(|(_, wind)| wind.speed_10m)
            .collect();

        let wind_gusts: Vec<f64> = day_data
            .iter()
            .filter_map(|(_, wind)| wind.gusts_10m)
            .collect();

        let wind_directions: Vec<f64> = day_data
            .iter()
            .filter_map(|(_, wind)| wind.direction_10m)
            .collect();

        let water_temps: Vec<f64> = day_data
            .iter()
            .filter_map(|(wave, _)| wave.sea_surface_temperature)
            .collect();

        let wave_height = average(&wave_heights);
        let swell_height = average(&swell_heights);
        let wind_speed = average(&wind_speeds);
        let wind_gust = maximum(&wind_gusts);
        let water_temp = average(&water_temps);
        let swell_deg = average(&swell_directions);
        let swell_period = average(&swell_periods);
        let wind_deg = average(&wind_directions);

        let has_wave_data = wave_height.is_some();

        let wave_height_m = wave_height
            .map(|value| format!("{value:.1}m"))
            .unwrap_or_else(|| "-".to_string());

        let swell_period_display = swell_period
            .map(|value| format!("{value:.1}s"))
            .unwrap_or_else(|| "-".to_string());

        let water_temp_display = water_temp
            .map(|value| format!("{value:.1}°"))
            .unwrap_or_else(|| "-".to_string());

        let wind_speed_display = wind_speed
            .map(|value| format!("{value:.0} kph"))
            .unwrap_or_else(|| "-".to_string());

        let wind_gust_display = wind_gust
            .map(|value| format!("{value:.0}"))
            .unwrap_or_else(|| "-".to_string());

        let swell_direction = swell_deg
            .map(degrees_to_direction)
            .unwrap_or_else(|| "-".to_string());

        let wind_direction = wind_deg
            .map(degrees_to_direction)
            .unwrap_or_else(|| "-".to_string());

        let rating = calculate_rating(wave_height, swell_height, wind_speed);

        result.push(SpotForecastDayView {
            day_name: header.day_name.clone(),
            date_str: header.date_str.clone(),
            is_today: header.is_today,

            has_data: has_wave_data,

            rating: rating.label.to_string(),
            rating_class: rating.class_name.to_string(),
            condition_tag: rating.tag.map(str::to_string),

            wave_height_m,

            swell_direction,
            swell_deg: swell_deg.unwrap_or(0.0),
            swell_period: swell_period_display,

            water_temp: water_temp_display,

            wind_speed: wind_speed_display,
            wind_gust: wind_gust_display,
            wind_direction,
            wind_deg: wind_deg.unwrap_or(0.0),

            air_temp: "-".to_string(),
        });
    }

    result
}

fn parse_forecast_date(value: &str) -> Option<NaiveDate> {
    value
        .split_once('T')
        .and_then(|(date, _)| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok())
}

pub fn build_day_headers(start_date: NaiveDate) -> Vec<DayHeaderView> {
    (0..7)
        .map(|offset| {
            let date = start_date + chrono::Duration::days(offset);

            DayHeaderView {
                day_name: date.format("%a").to_string().to_uppercase(),
                date_str: date.format("%d %b").to_string(),
                is_today: offset == 0,
            }
        })
        .collect()
}

fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn maximum(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::max)
}

fn degrees_to_direction(degrees: f64) -> String {
    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = ((degrees + 22.5) / 45.0).floor() as usize % 8;

    directions[index].to_string()
}

fn calculate_rating(
    wave_height: Option<f64>,
    swell_height: Option<f64>,
    wind_speed: Option<f64>,
) -> Rating {
    let wave = wave_height.unwrap_or(0.0);
    let swell = swell_height.unwrap_or(0.0);
    let wind = wind_speed.unwrap_or(0.0);

    if wave >= 1.5 && swell >= 1.0 && wind <= 20.0 {
        Rating {
            label: "EPIC",
            class_name: "epic",
            tag: Some("GO"),
        }
    } else if wave >= 1.0 && swell >= 0.7 && wind <= 25.0 {
        Rating {
            label: "GOOD",
            class_name: "good",
            tag: Some("GOOD"),
        }
    } else if wave >= 0.5 {
        Rating {
            label: "FAIR",
            class_name: "fair",
            tag: None,
        }
    } else {
        Rating {
            label: "POOR",
            class_name: "poor",
            tag: None,
        }
    }
}
