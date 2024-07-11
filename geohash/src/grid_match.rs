// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use crate::grid_generator::calculate_square_geohash;
use crate::area_generator::Area;

// Function to find the geohash for the nearest area (square) for given latitude and longitude
pub fn find_nearest_geohash_with_bounds(_lat: f64, _lon: f64, area: &Area) -> String {
    // Calculate the geohash for the area
    calculate_square_geohash(area)
}