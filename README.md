# Code repository for ICP Hackathon 

https://www.encode.club/icp-chain-fusion-hackathon



## Geohash Canister

### 1. Geohash Canister Logic

The Geohash Canister is designed to handle geolocation data and convert it into geohashes, as well as decode geohashes back into geographical coordinates. The canister provides two main functionalities:

- **compute_geohash**: This function takes a geolocation (latitude and longitude) as input and returns both the calculated geohash and the geographical bounds (square) that the geolocation falls within.
    - **Input**: Geolocation (latitude: f64, longitude: f64)
    - **Output**: (Area, String) where `Area` includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end) and `String` is the computed geohash.
    - **What it does**: The function computes the geohash for the provided geolocation and determines the geographical area (square bounds) that encompasses this geolocation.

- **compute_area**: This function takes a geohash as input and decodes it back into geographical coordinates. It then calculates the geographical bounds (square) for these coordinates.
    - **Input**: Geohash (String)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end) and the original geohash.
    - **What it does**: The function decodes the provided geohash into latitude and longitude, and then calculates the geographical area (square bounds) that encompasses these coordinates.

### 2. Tests

The testing suite for the Geohash Canister ensures the correctness and consistency of both the geohash computation and the geographical bounds determination. The key aspects tested include:

- **Geolocation to Geohash Conversion**: Verifies that the geohash computed for a given geolocation is correct.
- **Geohash to Square Bounds Conversion**: Ensures that the geographical area (square bounds) returned for a given geohash is accurate.
- **Consistency Within a Square**: Tests multiple geolocations within a specific square to confirm that the geohashes and square bounds remain consistent.

The tests are designed to check both the forward (geolocation to geohash) and reverse (geohash to square bounds) processes to ensure full coverage and reliability of the canister's functionalities.

## Comments
Please note:
This code is unfinished / in development