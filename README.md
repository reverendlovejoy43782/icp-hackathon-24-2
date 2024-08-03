# A Decentralized and tamper-proof data layer for the world (v2) 

## A Bitcoin-inspired version of Google Maps as a public good


### Map apps lack data for important geo decisions
Our understanding of the world’s surface increasingly comes from “second layers” like Google Maps. They inform decisions like “where to eat” or “where to buy.” However, they lack the information needed to decide “where to move,” “where to establish a business,” or “which neighborhood most needs public health measures.” These decisions require data on pollution rates over time, flood or hurricane probabilities, or major disease rates in a specific area. Furthermore, current second layers do not provide a way to communicate value directly tied to a location. This would enable compensating users for improving a place or donating to victims of a natural catastrophe in a specific region.


### We need trusted public memory and value layer of world’s surface
Just as Bitcoin revolutionized currency by making it a decentralized public good, we need a decentralized and tamper-proof public memory and value layer for the world. It is governed by a foundation or DAO and runs on the Internet Computer.

### Decentralized datalayer on ICP
This application uses a set of canisters on the Internet Computer to create a grid of fixed squares on the world’s surface. Each square can hold information and value. Users authenticate with Internet Identity and interact with squares via a React application to read or contribute information or communicate value tied to a specific square to other users.

It uses [geohashes](https://en.wikipedia.org/wiki/Geohash) to create a grid of squares of the worlds surface. Each square is an nft holding fixed information like IPNS name to store metadata on IPFS or crypto addresses like Bitcoin address and Ethereum address. The nft metadata points to changing information on IPFS like air quality index, crime rate or car accident rate. The application also interacts with blockchains to show value balances like Bitcoin or USDC or transact value.

Users authenticate with Internet Identity. They can contribute to a square by adding information to be compensated with some of the value held by the square. 



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

- **compute_geohash**: This function takes a geolocation (latitude and longitude) as input and returns both the calculated geohash and the geographical bounds (square) that the geolocation falls within. If an NFT for the square does not exist, it mints a new NFT, otherwise it retrieves the existing NFT information.
    - **Input**: Geolocation (latitude: f64, longitude: f64)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end), the computed geohash, the NFT information, and a flag indicating if the NFT was newly created or already existed.
    - **What it does**: The function computes the geohash for the provided geolocation, determines the geographical area (square bounds) that encompasses this geolocation, and handles NFT minting or retrieval.

- **compute_area**: This function takes a geohash as input and decodes it back into geographical coordinates. It then calculates the geographical bounds (square) for these coordinates. If an NFT for the square does not exist, it mints a new NFT, otherwise it retrieves the existing NFT information.
    - **Input**: Geohash (String)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end), the original geohash, the NFT information, and a flag indicating if the NFT was newly created or already existed.
    - **What it does**: The function decodes the provided geohash into latitude and longitude, calculates the geographical area (square bounds) that encompasses these coordinates, and handles NFT minting or retrieval.

## 2.2 Frontend Canister Logic

The Frontend Canister is responsible for providing the user interface to interact with the Geohash Canister. It allows users to input geolocation data or geohashes and visualize the corresponding geographical bounds on a map. The main functionalities include:

- **Fetching Geolocation**: Allows the user to fetch their current geolocation using the browser's geolocation API.
    - **Implementation**: A button triggers the geolocation fetch and updates the state with the retrieved latitude and longitude.

- **Submitting Geolocation**: Submits the provided geolocation to the Geohash Canister to compute the corresponding geohash, geographical bounds, and NFT information.
    - **Implementation**: Sends a request to `compute_geohash` with the geolocation and updates the map with the returned square bounds, geohash, and NFT information. Displays a message indicating if the NFT was newly created or already existed.

- **Submitting Geohash**: Submits the provided geohash to the Geohash Canister to compute the corresponding geographical bounds and NFT information.
    - **Implementation**: Sends a request to `compute_area` with the geohash and updates the map with the returned square bounds, geohash, and NFT information. Displays a message indicating if the NFT was newly created or already existed.

- **Displaying Map and Markers**: Visualizes the geographical bounds and geohash on a Google Map.
    - **Implementation**: Uses the Google Maps JavaScript API to render the map and markers. AdvancedMarkerElement is used to ensure future compatibility with Google Maps API.

- **Displaying NFT Information**: Shows the NFT details including owner, token ID, metadata, and content.
    - **Implementation**: Retrieves and displays the NFT information received from the Geohash Canister in a structured format.

## Comments
Please note:
This code is unfinished / in development