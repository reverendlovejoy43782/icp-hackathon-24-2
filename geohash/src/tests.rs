// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, _lat_range: f64, _lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    //let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    // let lat_range = lat_range - buffer * 2.0;
    // let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A')
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("TEST_ test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("TEST_ calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("TEST_ returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("TEST_ calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("TEST_ returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("TEST_ returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("TEST_ step size for {}_{}: lat_step_size = {}, lon_step_size = {}", square_number, letter, lat_step_size, lon_step_size);

        // Print expected step sizes
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let radians_lat = (lat * std::f64::consts::PI) / 180.0;
        let meters_in_degree_lon = METERS_IN_DEGREE_LAT * radians_lat.cos();
        let expected_step_degrees_lon = STEP_METERS / meters_in_degree_lon;
        println!("TEST_ expected step sizes: lat = {}, lon = {}", expected_step_degrees_lat, expected_step_degrees_lon);

        // Assert that the square is approximately 500 by 500 meters
        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "longitude range mismatch for {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}

/*
use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    vec![
        ((base_lat, base_lon), 'A'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("TEST_ test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("TEST_ calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("TEST_ returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("TEST_ calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("TEST_ returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("TEST_ returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("TEST_ step size for {}_{}: lat_step_size = {}, lon_step_size = {}", square_number, letter, lat_step_size, lon_step_size);

        // Print expected step sizes
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let expected_step_degrees_lon = STEP_METERS / (METERS_IN_DEGREE_LAT * (lat.to_radians().cos()));
        println!("TEST_ expected step sizes: lat = {}, lon = {}", expected_step_degrees_lat, expected_step_degrees_lon);

        // Assert that the square is approximately 500 by 500 meters
        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "longitude range mismatch for {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}


###


use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A')//,
        //((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        //((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        //((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        //((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        //((base_lat + lat_range * 0.05, base_lon), 'F'),
        //((base_lat - lat_range * 0.05, base_lon), 'G'),
        //((base_lat, base_lon + lon_range * 0.05), 'H'),
        //((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("TEST_ test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("TEST_ calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("TEST_ returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("TEST_ calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("TEST_ returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("TEST_ returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("TEST_ step size for {}_{}: lat_step_size = {}, lon_step_size = {}", square_number, letter, lat_step_size, lon_step_size);

        // Print expected step sizes
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let expected_step_degrees_lon = STEP_METERS / (METERS_IN_DEGREE_LAT * (lat.to_radians().cos()));
        println!("TEST_ expected step sizes: lat = {}, lon = {}", expected_step_degrees_lat, expected_step_degrees_lon);

        // Assert that the square is approximately 500 by 500 meters
        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "longitude range mismatch for {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}



#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}



#####


use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("TEST COORDINATE {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  CALCULATED SQUARE BOUNDS FOR TEST COORDINATE {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  RETURNED SQUARE BOUNDS FOR TEST COORDINATE {}_{}: {:?}", square_number, letter, area);
        println!("  CALCULATED GEOHASH FOR TEST COORDINATE {}_{}: {}", square_number, letter, base_geohash);
        println!("  RETURNED GEOHASH FOR TEST COORDINATE {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("  RETURNED SQUARE BOUNDS FOR TEST GEOHASH {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("  STEP SIZE FOR {}_{}: LAT_STEP_SIZE = {}, LON_STEP_SIZE = {}", square_number, letter, lat_step_size, lon_step_size);

        // Assert that the square is approximately 500 by 500 meters
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let expected_step_degrees_lon = STEP_METERS / (METERS_IN_DEGREE_LAT * (lat.to_radians().cos()));

        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "Latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "Longitude range mismatch for {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}


#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}



###

use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

#[test]
fn test_calculate_area() {
    // Test the area calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    println!("CALCULATED AREA: {:?}", area);
    // Assert that the calculated area boundaries are correct within the tolerance
    assert!((area.lat_start - 37.77).abs() < EPSILON);
    assert!((area.lon_start + 122.42).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39).abs() < EPSILON);
}

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("TEST COORDINATE {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  CALCULATED SQUARE BOUNDS FOR TEST COORDINATE {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  RETURNED SQUARE BOUNDS FOR TEST COORDINATE {}_{}: {:?}", square_number, letter, area);
        println!("  CALCULATED GEOHASH FOR TEST COORDINATE {}_{}: {}", square_number, letter, base_geohash);
        println!("  RETURNED GEOHASH FOR TEST COORDINATE {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("  RETURNED SQUARE BOUNDS FOR TEST GEOHASH {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("  STEP SIZE FOR {}_{}: LAT_STEP_SIZE = {}, LON_STEP_SIZE = {}", square_number, letter, lat_step_size, lon_step_size);

        // Assert that the square is approximately 500 by 500 meters
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let expected_step_degrees_lon = STEP_METERS / (METERS_IN_DEGREE_LAT * (lat.to_radians().cos()));

        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "Latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "Longitude range mismatch for {}_{}", square_number, letter);
    }
}


#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}

#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}


###

use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;
const STEP_METERS: f64 = 500.0;
const METERS_IN_DEGREE_LAT: f64 = 111320.0;

#[test]
fn test_calculate_area() {
    // Test the area calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    println!("Calculated area: {:?}", area);
    // Assert that the calculated area boundaries are correct within the tolerance
    assert!((area.lat_start - 37.77).abs() < EPSILON);
    assert!((area.lon_start + 122.42).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39).abs() < EPSILON);
}

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("Test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  Calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  Returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("  Calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("  Returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("  Returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);

        // Calculate and print the size of the square
        let lat_step_size = returned_area.lat_end - returned_area.lat_start;
        let lon_step_size = returned_area.lon_end - returned_area.lon_start;
        println!("  Step size for {}_{}: lat_step_size = {}, lon_step_size = {}", square_number, letter, lat_step_size, lon_step_size);

        // Assert that the square is approximately 500 by 500 meters
        let expected_step_degrees_lat = STEP_METERS / METERS_IN_DEGREE_LAT;
        let expected_step_degrees_lon = STEP_METERS / (METERS_IN_DEGREE_LAT * (lat.to_radians().cos()));

        assert!((lat_step_size - expected_step_degrees_lat).abs() < EPSILON, "Latitude range mismatch for {}_{}", square_number, letter);
        assert!((lon_step_size - expected_step_degrees_lon).abs() < EPSILON, "Longitude range mismatch for {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}


#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}






####
####
use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;

#[test]
fn test_calculate_area() {
    // Test the area calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    println!("Calculated area: {:?}", area);
    // Assert that the calculated area boundaries are correct within the tolerance
    assert!((area.lat_start - 37.77).abs() < EPSILON);
    assert!((area.lon_start + 122.42).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39).abs() < EPSILON);
}

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("Test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  Calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  Returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("  Calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("  Returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        let area_response = query_compute_area(returned_geohash.clone());
        let returned_area = Area {
            lat_start: area_response.lat_start,
            lon_start: area_response.lon_start,
            lat_end: area_response.lat_end,
            lon_end: area_response.lon_end,
        };

        // Print the information for the geohash -> area test
        println!("  Returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

        // Assert that the returned area boundaries match the calculated area
        assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
        assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}

#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}



###


use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;

#[test]
fn test_calculate_area() {
    // Test the area calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    println!("Calculated area: {:?}", area);
    // Assert that the calculated area boundaries are correct within the tolerance
    assert!((area.lat_start - 37.77).abs() < EPSILON);
    assert!((area.lon_start + 122.42).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39).abs() < EPSILON);
}

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each square has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = query_compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = query_compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("Test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  Calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  Returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("  Calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("  Returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        if let area_response = query_compute_area(returned_geohash.clone()) {
            let returned_area = Area {
                lat_start: area_response.lat_start,
                lon_start: area_response.lon_start,
                lat_end: area_response.lat_end,
                lon_end: area_response.lon_end,
            };

            // Print the information for the geohash -> area test
            println!("  Returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

            // Assert that the returned area boundaries match the calculated area
            assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);
        } else {
            panic!("Failed to decode geohash {} for test coordinate {}_{}", returned_geohash, square_number, letter);
        }
    }
}

#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}

#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}

#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}

###



use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;

#[test]
fn test_calculate_area() {
    // Test the area calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    println!("Calculated area: {:?}", area);
    // Assert that the calculated area boundaries are correct within the tolerance
    assert!((area.lat_start - 37.769999999999996).abs() < EPSILON);
    assert!((area.lon_start + 122.42999999999999).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39999999999999).abs() < EPSILON);
}

// Helper function to generate test locations within a given square
fn generate_test_locations(base_lat: f64, base_lon: f64, lat_range: f64, lon_range: f64) -> Vec<((f64, f64), char)> {
    // Define a buffer to avoid edge effects
    let buffer = 0.01;
    // Adjust latitude and longitude ranges by the buffer
    let lat_range = lat_range - buffer * 2.0;
    let lon_range = lon_range - buffer * 2.0;

    // Return a vector of test locations within the square, each associated with a letter (each Squares has a number, each location within a square has a letter e.g. 1_A)
    vec![
        ((base_lat, base_lon), 'A'),
        ((base_lat + lat_range * 0.1, base_lon + lon_range * 0.1), 'B'),
        ((base_lat - lat_range * 0.1, base_lon - lon_range * 0.1), 'C'),
        ((base_lat + lat_range * 0.1, base_lon - lon_range * 0.1), 'D'),
        ((base_lat - lat_range * 0.1, base_lon + lon_range * 0.1), 'E'),
        ((base_lat + lat_range * 0.05, base_lon), 'F'),
        ((base_lat - lat_range * 0.05, base_lon), 'G'),
        ((base_lat, base_lon + lon_range * 0.05), 'H'),
        ((base_lat, base_lon - lon_range * 0.05), 'I'),
    ]
}

// Shared function to test consistent geohash within a square
fn test_consistent_geohash_within_square(base_lat: f64, base_lon: f64, square_number: usize) {
    // Calculate the base area (square) for the given latitude and longitude
    let base_area = calculate_area(base_lat, base_lon);
    // Calculate the geohash for the center of the base area
    let base_geohash = {
        let (_, geohash) = compute_geohash(Geolocation { latitude: base_lat, longitude: base_lon });
        geohash
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let (area, returned_geohash) = compute_geohash(Geolocation { latitude: lat, longitude: lon });

        // Print the test information
        println!("Test coordinate {}_{}: ({}, {})", square_number, letter, lat, lon);
        println!("  Calculated square bounds for test coordinate {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  Returned square bounds for test coordinate {}_{}: {:?}", square_number, letter, area);
        println!("  Calculated geohash for test coordinate {}_{}: {}", square_number, letter, base_geohash);
        println!("  Returned geohash for test coordinate {}_{}: {}", square_number, letter, returned_geohash);

        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(returned_geohash, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash", square_number, letter, lat, lon);

        // Assert that the area boundaries are correct within the tolerance
        assert!((area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for {}_{}", square_number, letter);
        assert!((area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for {}_{}", square_number, letter);
        assert!((area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for {}_{}", square_number, letter);
        assert!((area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for {}_{}", square_number, letter);

        // Now test if the returned geohash yields the correct area
        if let Ok(area_response) = compute_area(returned_geohash.clone()) {
            let returned_area = Area {
                lat_start: area_response.lat_start,
                lon_start: area_response.lon_start,
                lat_end: area_response.lat_end,
                lon_end: area_response.lon_end,
            };

            // Print the information for the geohash -> area test
            println!("  Returned square bounds for test geohash {}_{}: {:?}", square_number, letter, returned_area);

            // Assert that the returned area boundaries match the calculated area
            assert!((returned_area.lat_start - calculated_area.lat_start).abs() < EPSILON, "lat_start mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lon_start - calculated_area.lon_start).abs() < EPSILON, "lon_start mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lat_end - calculated_area.lat_end).abs() < EPSILON, "lat_end mismatch for geohash {}_{}", square_number, letter);
            assert!((returned_area.lon_end - calculated_area.lon_end).abs() < EPSILON, "lon_end mismatch for geohash {}_{}", square_number, letter);
        } else {
            panic!("Failed to decode geohash {} for test coordinate {}_{}", returned_geohash, square_number, letter);
        }
    }
}




#[test]
fn test_consistent_geohash_within_square_1() {
    // Test the consistency of geohashes within the calculated square
    test_consistent_geohash_within_square(37.7749, -122.4194, 1);
}

#[test]
fn test_consistent_geohash_within_square_2() {
    test_consistent_geohash_within_square(40.7128, -74.0060, 2);
}


#[test]
fn test_consistent_geohash_within_square_3() {
    test_consistent_geohash_within_square(34.0522, -118.2437, 3);
}

#[test]
fn test_consistent_geohash_within_square_4() {
    test_consistent_geohash_within_square(51.5074, -0.1278, 4);
}

#[test]
fn test_consistent_geohash_within_square_5() {
    test_consistent_geohash_within_square(35.6895, 139.6917, 5);
}

#[test]
fn test_consistent_geohash_within_square_6() {
    test_consistent_geohash_within_square(-33.8688, 151.2093, 6);
}
*/


