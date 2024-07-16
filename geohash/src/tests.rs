// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use crate::{query_compute_geohash, Geolocation, AreaResponse};

// Define a set of test geolocations covering the globe
const TEST_GEOLOCATIONS: [(f64, f64); 25] = [
    (37.7749, -122.4194), // San Francisco, USA
    (51.5074, -0.1278),   // London, UK
    (35.6895, 139.6917),  // Tokyo, Japan
    (-33.8688, 151.2093), // Sydney, Australia
    (48.8566, 2.3522),    // Paris, France
    (40.7128, -74.0060),  // New York, USA
    (55.7558, 37.6173),   // Moscow, Russia
    (-23.5505, -46.6333), // São Paulo, Brazil
    (19.0760, 72.8777),   // Mumbai, India
    (34.0522, -118.2437), // Los Angeles, USA
    (-26.2041, 28.0473),  // Johannesburg, South Africa
    (39.9042, 116.4074),  // Beijing, China
    (1.3521, 103.8198),   // Singapore
    (35.6762, 139.6503),  // Tokyo, Japan (different location)
    (41.9028, 12.4964),   // Rome, Italy
    (34.6937, 135.5023),  // Osaka, Japan
    (22.3964, 114.1095),  // Hong Kong
    (55.7558, 37.6173),   // Moscow, Russia
    (37.5665, 126.9780),  // Seoul, South Korea
    (30.0444, 31.2357),   // Cairo, Egypt
    (-34.6037, -58.3816), // Buenos Aires, Argentina
    (50.8503, 4.3517),    // Brussels, Belgium
    (40.4168, -3.7038),   // Madrid, Spain
    (52.5200, 13.4050),   // Berlin, Germany
    (19.4326, -99.1332),  // Mexico City, Mexico
];

fn validate_geolocation_in_area(lat: f64, lon: f64, area: &AreaResponse) {
    assert!(lat >= area.lat_start && lat <= area.lat_end, "Latitude is not within the area bounds");
    assert!(lon >= area.lon_start && lon <= area.lon_end, "Longitude is not within the area bounds");
}

#[test]
fn test_geolocation_within_square() {
    for &(lat, lon) in &TEST_GEOLOCATIONS {
        let geolocation = Geolocation { latitude: lat, longitude: lon };
        let area_response: AreaResponse = query_compute_geohash(geolocation);
        validate_geolocation_in_area(lat, lon, &area_response);
    }
}