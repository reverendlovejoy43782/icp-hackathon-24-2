// SPDX-License: MIT
// (C) 2024 Thomas Magerl

use crate::area_generator::Area;
use geohash::{encode, decode, Coord};

// Function to decode a geohash back into coordinates
pub fn decode_geohash(geohash: &str) -> Result<Coord, &'static str> {
    decode(geohash).map(|(coord, _, _)| coord).map_err(|_| "Invalid geohash")
}

// Function to calculate the bounds of a square based on its center and step sizes
fn calculate_square_bounds(center_lat: f64, center_lon: f64, lat_step: f64, lon_step: f64) -> Area {
    let half_lat = lat_step / 2.0;
    let half_lon = lon_step / 2.0;

    Area {
        lat_start: center_lat - half_lat,
        lon_start: center_lon - half_lon,
        lat_end: center_lat + half_lat,
        lon_end: center_lon + half_lon,
    }
}

// Function to create a grid with geohashes within given latitude and longitude bounds
pub fn create_grid_with_geohash(lat_start: f64, lon_start: f64, lat_end: f64, lon_end: f64, step_meters: f64) -> Vec<(f64, f64, String, Area)> {
    // Conversion factor from meters to degrees latitude
    const METERS_IN_DEGREE_LAT: f64 = 111320.0;

    let mut grid_points_with_geohash = Vec::new();

    // Calculate step size in degrees for latitude
    let step_degrees_lat = step_meters / METERS_IN_DEGREE_LAT;

    // Populate latitudes within the specified range
    let mut latitudes = Vec::new();
    let mut lat = lat_start;
    while lat <= lat_end {
        latitudes.push(lat);
        lat += step_degrees_lat;
    }

    for &lat in &latitudes {
        // Convert latitude from degrees to radians for accurate calculation
        let radians_lat = (lat * std::f64::consts::PI) / 180.0;
        // Conversion factor from meters to degrees longitude, adjusted for this latitude
        let meters_in_degree_lon = METERS_IN_DEGREE_LAT * radians_lat.cos();
        // Calculate step size in degrees for longitude
        let step_degrees_lon = step_meters / meters_in_degree_lon;

        let mut longitudes = Vec::new();
        let mut lon = lon_start;
        while lon <= lon_end {
            longitudes.push(lon);
            lon += step_degrees_lon;
        }

        for &lon in &longitudes {
            // Calculate the center point of each grid square
            let center_lat = lat + step_degrees_lat / 2.0;
            let center_lon = lon + step_degrees_lon / 2.0;

            // Generate geohash for the center point
            let geohash_key = encode(Coord { x: center_lon, y: center_lat }, 12).unwrap();

            // Calculate bounds for the square
            let square_bounds = calculate_square_bounds(center_lat, center_lon, step_degrees_lat, step_degrees_lon);

            // Print the details of the area and the square
            /*
            println!("GRID_GENERATOR_ area bounds: lat_start = {}, lon_start = {}, lat_end = {}, lon_end = {}", lat_start, lon_start, lat_end, lon_end);
            println!("GRID_GENERATOR_ step size in degrees: lat_step = {}", step_degrees_lat);
            println!("GRID_GENERATOR_ latitude: {}, step size in degrees for longitude: {}", lat, step_degrees_lon);
            println!("GRID_GENERATOR_ square center: ({}, {}), geohash: {}, bounds: {:?}", center_lat, center_lon, geohash_key, square_bounds);
            */
            // Add the grid point with its geohash and bounds to the array
            grid_points_with_geohash.push((center_lat, center_lon, geohash_key, square_bounds));
        }
    }

    grid_points_with_geohash
}

