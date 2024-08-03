# A data layer for the world (v2) 
This is a basic version of a decentralized application built on the Internet Computer. It creates a geo-information platform that serves as a public good for everyone.

## We need a Bitcoin-inspired version of Google Maps as a public good
Our understanding of the world’s surface increasingly comes from “second layers” like Google Maps. These layers are used by humans, cars, and other devices for navigation.

### Map apps lack data for important geo decisions
Current platforms inform decisions like “where to eat?” or “where to buy?” However, they lack the information needed to decide “where to move,” “where to avoid due to high crime rates,” or “which neighborhood most needs public health measures.” These decisions require data on pollution rates over time, flood or hurricane probabilities, and crime rates in specific areas. Furthermore, current second layers do not provide a way to communicate value directly tied to a location. This would enable compensating users for improving an area or donating to victims of a natural catastrophe in a specific region.


### We need a trusted public memory and value layer of world’s surface
Just as Bitcoin revolutionized currency by making it a decentralized public good, we need a decentralized and tamper-proof public memory and value layer for the world. It is governed by a foundation or DAO and runs on the Internet Computer.

## This application ...
... uses a set of canisters on the Internet Computer to create a grid of the world’s surface. Each square in the grid can hold information and value. Users authenticate with Internet Identity and interact with the squares via a React application to read or contribute information or communicate value tied to a specific square to other users.

Users can send value to one of the square’s addresses or contribute to a square by adding information, which can be compensated with some of the value held by the square. When a square is first visited by an authenticated user, its NFT is transferred to the NFT wallet of that user. Possession of a square’s NFT does not give any control over the square; it is similar to owning an NFT of digital art. NFT owners are patrons of the NFT and are encouraged to improve its health metrics and gather donations. They earn part of the square’s income when its health and donation flow are good.

## Under the hood it ...
... creates fixed squares where each square is represented by a [geohashe](https://en.wikipedia.org/wiki/Geohash). Each square / geohash is a [Dip721 Nft](https://github.com/Psychedelic/DIP721/blob/develop/spec.md) holding information such as an IPNS name pointing to metadata on IPFS. It also holds crypto addresses for Bitcoin, Ether, or USDC. The NFT points to changing information on IPFS like air quality index, crime rate, or car accident rate. This information is regularly updated via APIs or user contributions. The application interacts with Bitcoin or EVM blockchains to show balances and transact Bitcoin, Ether, or USDC between users and squares.

## Architecture

### frontend canister

### basic_bitcoin canister

### basic_ethereum canister

### dip721_nft_container

### geohash canister




# 2. Tech

## 2.1 Geohash Canister

The Geohash Canister is designed to handle geolocation data and convert it into geohashes, as well as decode geohashes back into geographical coordinates. It serves as the backend for the frontend, handling calls to the dip721_nft_container, basic_bitcoin, and basic_ethereum canisters to mint or look up NFTs, and query information about balances or metadata on IPFS. Some of this functionality (Ethereum balance, IPFS interaction) is not part of this MVP though. 

The canister provides three main functionalities:

- **compute_geohash**: This function takes a geolocation (latitude and longitude) as input and returns both the calculated geohash and the geographical bounds (square) that the geolocation falls within. If an NFT for the square does not exist, it mints a new NFT; otherwise, it retrieves the existing NFT information.
    - **Input**: Geolocation (latitude: f64, longitude: f64)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end), the computed geohash, the NFT information, Bitcoin and Ethereum balances, real-time metrics, and a flag indicating if the NFT was newly created or already existed.
    - **What it does**: The function computes the geohash for the provided geolocation, determines the geographical area (square bounds) that encompasses this geolocation, handles NFT minting or retrieval, and provides real-time metrics and cryptocurrency balances.

- **compute_area**: This function takes a geohash as input and decodes it back into geographical coordinates. It then calculates the geographical bounds (square) for these coordinates. If an NFT for the square does not exist, it mints a new NFT; otherwise, it retrieves the existing NFT information.
    - **Input**: Geohash (String)
    - **Output**: AreaResponse which includes the latitude and longitude boundaries (lat_start, lon_start, lat_end, lon_end), the original geohash, the NFT information, Bitcoin and Ethereum balances, real-time metrics, and a flag indicating if the NFT was newly created or already existed.
    - **What it does**: The function decodes the provided geohash into latitude and longitude, calculates the geographical area (square bounds) that encompasses these coordinates, handles NFT minting or retrieval, and provides real-time metrics and cryptocurrency balances.

- **update_rating**: This function allows users to update the rating of a specific square, serving as a simple MVP for user-contributed information about a square.
    - **Input**: IPNS name (String), Rating (u32)
    - **Output**: Result indicating success or failure
    - **What it does**: The function updates the rating for the specified square, provided the rating is within the valid range (1 to 10).

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