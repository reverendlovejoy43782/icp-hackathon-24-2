// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use super::*;

// Define a tolerance value for floating-point comparisons
const EPSILON: f64 = 1e-6;

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
        let nearest = find_nearest_geohash_with_bounds(base_lat, base_lon, &base_area);
        nearest
    };

    // Generate test locations within the base area
    let test_locations = generate_test_locations(base_lat, base_lon, base_area.lat_end - base_area.lat_start, base_area.lon_end - base_area.lon_start);

    // Iterate over each test location and its associated letter
    for ((lat, lon), letter) in test_locations {
        // Calculate the area for the test location
        let calculated_area = calculate_area(lat, lon);
        // Find the nearest geohash for the test location
        let nearest = find_nearest_geohash_with_bounds(lat, lon, &calculated_area);
        // Print the test information

        println!("Test location {}_{} ({}, {}) in Square {}:", square_number, letter, lat, lon, square_number);
        println!("  Calculated Square bounds {}_{}: {:?}", square_number, letter, calculated_area);
        println!("  Test Square bounds {}_{}: {:?}", square_number, letter, base_area);
        println!("  Geohash for test location {}_{}: {}", square_number, letter, nearest);
        // Assert that the geohash for the test location matches the base geohash
        assert_eq!(nearest, base_geohash, "Geohash for location {}_{} ({}, {}) did not match base geohash (Square {})", square_number, letter, lat, lon, square_number);
    }

}

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

#[test]
fn test_find_nearest_geohash_with_bounds() {
    // Test the nearest geohash calculation for a specific latitude and longitude
    let area = calculate_area(37.7749, -122.4194);
    let nearest = find_nearest_geohash_with_bounds(37.7749, -122.4194, &area);
    println!("Nearest geohash: {}", nearest);
    // Assert that the geohash length is 12
    assert_eq!(nearest.len(), 12);
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