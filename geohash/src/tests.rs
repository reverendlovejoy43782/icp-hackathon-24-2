// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

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



/*
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
    }
}
*/

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


//////

/*

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
fn test_compute_geohash() {
    let geolocation = Geolocation { latitude: 37.7749, longitude: -122.4194 };
    let (area, geohash) = compute_geohash(geolocation);

    // Verify the returned area
    assert!((area.lat_start - 37.769999999999996).abs() < EPSILON);
    assert!((area.lon_start + 122.42999999999999).abs() < EPSILON);
    assert!((area.lat_end - 37.8).abs() < EPSILON);
    assert!((area.lon_end + 122.39999999999999).abs() < EPSILON);

    // Verify the geohash length
    assert_eq!(geohash.len(), 12);
}

#[test]
fn test_compute_area_from_geohash() {
    let geohash = "9q8yyqs1ef80";
    let area_response = compute_area(geohash.to_string()).unwrap();

    // Verify the returned area
    assert!((area_response.lat_start - 37.769999999999996).abs() < EPSILON);
    assert!((area_response.lon_start + 122.42999999999999).abs() < EPSILON);
    assert!((area_response.lat_end - 37.8).abs() < EPSILON);
    assert!((area_response.lon_end + 122.39999999999999).abs() < EPSILON);

    // Verify the returned geohash
    assert_eq!(area_response.geohash, geohash);
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
*/