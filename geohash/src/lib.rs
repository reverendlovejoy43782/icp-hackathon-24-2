// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

mod area_generator;
mod grid_generator;
mod grid_match;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use area_generator::calculate_area;
use grid_match::find_nearest_geohash_with_bounds;
use grid_generator::decode_geohash;
use crate::area_generator::Area;

#[query(name = "compute_geohash")]
fn candid_compute_geohash(geolocation: Geolocation) -> (Area, String) {
    compute_geohash(geolocation)
}

#[query(name = "compute_area")]
fn candid_compute_area(geohash: String) -> Result<AreaResponse, String> {
    compute_area(geohash)
}

// Define a struct for geolocation with latitude and longitude
#[derive(CandidType, Deserialize)]
struct Geolocation {
    latitude: f64,
    longitude: f64,
}


// Define an update function to compute the area and geohash for a given geolocation
#[update]
fn compute_geohash(geolocation: Geolocation) -> (Area, String) {
    // Calculate the area for the given geolocation
    let area = calculate_area(geolocation.latitude, geolocation.longitude);

    // Find the nearest geohash within the calculated area
    let nearest = find_nearest_geohash_with_bounds(geolocation.latitude, geolocation.longitude, &area);

    // Print the geohash
    ic_cdk::println!("Computed geohash: {}", nearest);

    // Return the area and the nearest geohash
    (area, nearest)
}

/*
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
*/


// Define a struct for area response to be used with Candid
#[derive(CandidType, Deserialize)]
struct AreaResponse {
    lat_start: f64,
    lon_start: f64,
    lat_end: f64,
    lon_end: f64,
    geohash: String,
}

// Define an update function to compute the area for a given geohash
#[update]
fn compute_area(geohash: String) -> Result<AreaResponse, String> {
    // Decode the geohash back into coordinates
    let coord = decode_geohash(&geohash).map_err(|err| err.to_string())?;

    // Calculate the area for the given coordinates
    let area = calculate_area(coord.y, coord.x);

    // Print the decoded coordinates and area
    ic_cdk::println!("Decoded coordinates: ({}, {})", coord.y, coord.x);
    ic_cdk::println!("Calculated area: {:?}", area);

    // Return the area and the original geohash in AreaResponse
    Ok(AreaResponse {
        lat_start: area.lat_start,
        lon_start: area.lon_start,
        lat_end: area.lat_end,
        lon_end: area.lon_end,
        geohash,
    })
}

// Include the tests module for unit tests
#[cfg(test)]
mod tests;