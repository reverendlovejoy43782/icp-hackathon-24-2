# Code repository for ICP Hackathon 

https://www.encode.club/icp-chain-fusion-hackathon


# 1. Usage

- Decentralised data layer of the world
- The world is divided into fixed 500 x 500 meter squares
- Each square can hold information, such as events or conditions specific to that location
- Each square can hold value, such as USDC, Ether, or Bitcoin
- Users can interact with squares by reading the information, adding value to a square, and contributing information about a location within a square
- Value is distributed to users on the square e.g. for contributing information

# 2. Tech

## 2.1 Geohash Canister


The Geohash Canister is designed to handle geolocation data and convert it into geohashes, as well as decode geohashes back into geographical coordinates. The canister provides two main functionalities:

- **compute_geohash**: This function takes a geolocation (latitude and longitude) as input and returns both the calculated geohash and the geographical bounds (square) that the geolocation falls within.
    - **Input**: Geolocation (latitude: f64, longitude: f64)
    - **Output**: (Area, String) where `Area` includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end) and `String` is the computed geohash.
    - **What it does**: The function computes the geohash for the provided geolocation and determines the geographical area (square bounds) that encompasses this geolocation.

- **compute_area**: This function takes a geohash as input and decodes it back into geographical coordinates. It then calculates the geographical bounds (square) for these coordinates.
    - **Input**: Geohash (String)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end) and the original geohash.
    - **What it does**: The function decodes the provided geohash into latitude and longitude, and then calculates the geographical area (square bounds) that encompasses these coordinates.


## 2.2 Frontend Canister Logic

The Frontend Canister is responsible for providing the user interface to interact with the Geohash Canister. It allows users to input geolocation data or geohashes and visualize the corresponding geographical bounds on a map. The main functionalities include:

- **Fetching Geolocation**: Allows the user to fetch their current geolocation using the browser's geolocation API.
    - **Implementation**: A button triggers the geolocation fetch and updates the state with the retrieved latitude and longitude.

- **Submitting Geolocation**: Submits the provided geolocation to the Geohash Canister to compute the corresponding geohash and geographical bounds.
    - **Implementation**: Sends a request to `query_compute_geohash` with the geolocation and updates the map with the returned square bounds and geohash.

- **Submitting Geohash**: Submits the provided geohash to the Geohash Canister to compute the corresponding geographical bounds.
    - **Implementation**: Sends a request to `query_compute_area` with the geohash and updates the map with the returned square bounds.

- **Displaying Map and Markers**: Visualizes the geographical bounds and geohash on a Google Map.
    - **Implementation**: Uses the Google Maps JavaScript API to render the map and markers. AdvancedMarkerElement is used to ensure future compatibility with Google Maps API.

## Comments
Please note:
This code is unfinished / in development