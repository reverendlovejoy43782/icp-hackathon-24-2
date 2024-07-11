// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

mod area_generator;
mod grid_generator;
mod grid_match;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use area_generator::calculate_area;
use grid_match::find_nearest_geohash_with_bounds;

// Define a struct for geolocation with latitude and longitude
#[derive(CandidType, Deserialize)]
struct Geolocation {
    latitude: f64,
    longitude: f64,
}

// Define an update function to compute geohash for a given geolocation
#[update]
fn compute_geohash(geolocation: Geolocation) -> String {
    // Calculate the area for the given geolocation
    let area = calculate_area(geolocation.latitude, geolocation.longitude);

    // Find the nearest geohash within the calculated area
    let nearest = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude, &area);

    // Print the geohash
    ic_cdk::println!("Computed geohash: {}", nearest);

    // Return the nearest geohash
    nearest
}


// Include the tests module for unit tests
#[cfg(test)]
mod tests;