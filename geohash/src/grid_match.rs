// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use crate::grid_generator::create_grid_with_geohash;
use crate::area_generator::Area;

/// Function to find the geohash for the nearest area (square) for given latitude and longitude
pub fn find_nearest_geohash_with_bounds(lat: f64, lon: f64) -> (String, Area) {
    let step_meters = 500.0;

    // Calculate the grid boundaries
    let area = crate::area_generator::calculate_area(lat, lon);

    // Generate grid points with their corresponding geohashes and bounds within these boundaries
    let grid_points_with_geohash = create_grid_with_geohash(area.lat_start, area.lon_start, area.lat_end, area.lon_end, step_meters);

    let mut closest_geohash = String::new();
    let mut closest_bounds = Area {
        lat_start: 0.0,
        lon_start: 0.0,
        lat_end: 0.0,
        lon_end: 0.0,
    };
    let mut min_distance = f64::MAX;

    // Iterate through each grid point to find the nearest one
    for (center_lat, center_lon, geohash_key, square_bounds) in grid_points_with_geohash {
        // Calculate Euclidean distance to the given location
        let distance = ((center_lat - lat).powi(2) + (center_lon - lon).powi(2)).sqrt();

        if distance < min_distance {
            min_distance = distance;
            closest_geohash = geohash_key;
            closest_bounds = square_bounds;
        }
    }

    // Return the nearest geohash and its bounds
    (closest_geohash, closest_bounds)
}
