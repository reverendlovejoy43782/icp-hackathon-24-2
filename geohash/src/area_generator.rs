// SPDX-License-Identifier: MIT
// (C) 2024 Thomas Magerl

use ic_cdk::export::candid::{CandidType, Deserialize};


#[derive(Debug, CandidType, Deserialize)]
pub struct Area {
    pub lat_start: f64,
    pub lon_start: f64,
    pub lat_end: f64,
    pub lon_end: f64,
}


pub fn calculate_area(lat: f64, lon: f64) -> Area {
    // Set grid spacing to 0.03 degrees for both latitude and longitude
    let grid_spacing = 0.03;

    // Calculate the nearest lower latitude line based on the grid spacing
    let lower_lat = (lat / grid_spacing).floor() * grid_spacing;

    // Calculate the nearest upper latitude line by adding grid spacing to the lower latitude
    let upper_lat = lower_lat + grid_spacing;

    // Calculate the nearest lower longitude line based on the grid spacing
    let lower_lon = (lon / grid_spacing).floor() * grid_spacing;

    // Calculate the nearest upper longitude line by adding grid spacing to the lower longitude
    let upper_lon = lower_lon + grid_spacing;

    // Create the area
    let area = Area {
        lat_start: lower_lat,
        lon_start: lower_lon,
        lat_end: upper_lat,
        lon_end: upper_lon,
    };

    // Print the calculated area
    println!("AREA_GENERATOR_area: {:?}", area);

    area
}

/*
pub fn calculate_area(lat: f64, lon: f64) -> Area {
    // 500 meters in degrees
    let half_side_length = 500.0 / METERS_IN_DEGREE;

    Area {
        lat_start: lat - half_side_length,
        lon_start: lon - half_side_length,
        lat_end: lat + half_side_length,
        lon_end: lon + half_side_length,
    }
}


###

use ic_cdk::export::candid::{CandidType, Deserialize};


#[derive(Debug, CandidType, Deserialize)]
pub struct Area {
    pub lat_start: f64,
    pub lon_start: f64,
    pub lat_end: f64,
    pub lon_end: f64,
}



pub fn calculate_area(lat: f64, lon: f64) -> Area {
    let grid_spacing = 0.03;

    let lower_lat = (lat / grid_spacing).floor() * grid_spacing;
    let upper_lat = lower_lat + grid_spacing;

    let lower_lon = (lon / grid_spacing).floor() * grid_spacing;
    let upper_lon = lower_lon + grid_spacing;


    Area {
        lat_start: lower_lat,
        lon_start: lower_lon,
        lat_end: upper_lat,
        lon_end: upper_lon,
    }
}
*/



