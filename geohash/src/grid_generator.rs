// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use geohash::{encode, Coord};
use crate::area_generator::Area;

// Function to calculate the geohash for the center of a given area
pub fn calculate_square_geohash(area: &Area) -> String {
    // Calculate the center latitude of the area
    let center_lat = (area.lat_start + area.lat_end) / 2.0;
    // Calculate the center longitude of the area
    let center_lon = (area.lon_start + area.lon_end) / 2.0;
    // Encode the center coordinates into a geohash with precision 12
    encode(Coord { x: center_lon, y: center_lat }, 12).unwrap()
}