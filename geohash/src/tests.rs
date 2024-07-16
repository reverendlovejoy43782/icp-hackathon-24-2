// SPDX-License: MIT
// (C) 2024 Thomas Magerl

use crate::{query_compute_geohash, Geolocation, AreaResponse};
use rand::Rng;
use std::fmt::{Debug, Formatter, Result};

// Ensure AreaResponse implements Debug trait for debugging
impl Debug for AreaResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("AreaResponse")
            .field("lat_start", &self.lat_start)
            .field("lon_start", &self.lon_start)
            .field("lat_end", &self.lat_end)
            .field("lon_end", &self.lon_end)
            .field("geohash", &self.geohash)
            .finish()
    }
}

// Function to generate 100 geolocations spread across the globe
fn generate_test_geolocations() -> Vec<((f64, f64), usize)> {
    let mut geolocations = Vec::new();
    let mut rng = rand::thread_rng();
    
    for i in 0..10 {
        for j in 0..10 {
            let lat = -90.0 + i as f64 * 18.0 + rng.gen_range(0.0..18.0);  // Randomize within each 18-degree segment
            let lon = -180.0 + j as f64 * 36.0 + rng.gen_range(0.0..36.0);  // Randomize within each 36-degree segment
            geolocations.push(((lat, lon), i * 10 + j + 1));
        }
    }

    geolocations
}

fn validate_geolocation_in_area(lat: f64, lon: f64, area: &AreaResponse, test_case_number: usize) {
    println!("Test case {}: Latitude = {}, Area.lat_start = {}, Area.lat_end = {}", test_case_number, lat, area.lat_start, area.lat_end);
    println!("Test case {}: Longitude = {}, Area.lon_start = {}, Area.lon_end = {}", test_case_number, lon, area.lon_start, area.lon_end);
    assert!(lat >= area.lat_start && lat <= area.lat_end, "Test case {}: Latitude is not within the area bounds", test_case_number);
    assert!(lon >= area.lon_start && lon <= area.lon_end, "Test case {}: Longitude is not within the area bounds", test_case_number);
}

#[test]
fn test_geolocation_within_square() {
    let test_geolocations = generate_test_geolocations();
    
    for &((lat, lon), test_case_number) in &test_geolocations {
        let geolocation = Geolocation { latitude: lat, longitude: lon };
        let area_response: AreaResponse = query_compute_geohash(geolocation);
        println!("Test case {}: Geolocation = ({}, {}), Area = {:?}", test_case_number, lat, lon, area_response);
        validate_geolocation_in_area(lat, lon, &area_response, test_case_number);
    }
}
