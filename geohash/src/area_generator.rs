// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl




pub fn calculate_area(lat: f64, lon: f64) -> Area {
    let grid_spacing = 0.03;

    let lower_lat = (lat / grid_spacing).floor() * grid_spacing;
    let upper_lat = lower_lat + grid_spacing;

    let lower_lon = (lon / grid_spacing).floor() * grid_spacing;
    let upper_lon = lower_lon + grid_spacing;


    Area {
        lat_start: lower_lat,
        lon_start: lower_lon,
        lat_end: upper_lat,
        lon_end: upper_lon,
    }
}

#[derive(Debug)]
pub struct Area {
    pub lat_start: f64,
    pub lon_start: f64,
    pub lat_end: f64,
    pub lon_end: f64,
}