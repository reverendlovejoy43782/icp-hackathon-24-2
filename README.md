# A data layer for the world (v2) 
This is a basic version of a decentralized application built on the Internet Computer. It creates a geo-information platform that serves as a public good for everyone.

## We need a Bitcoin-inspired version of Google Maps as a public good
Our understanding of the world’s surface increasingly comes from “second layers” like Google Maps. These layers are used by humans, cars, and other devices for navigation.

### Map apps lack data for important geo decisions
Current platforms inform decisions like “where to eat?” or “where to buy?” However, they lack the information needed to decide “where to move,” “where to avoid due to high crime rates,” or “which neighborhood most needs public health measures.” These decisions require data on pollution rates over time, flood or hurricane probabilities, and crime rates in specific areas. Furthermore, current second layers do not provide a way to communicate value directly tied to a location. This would enable compensating users for improving an area or donating to victims of a natural catastrophe in a specific region.


### We need a trusted public memory and value layer of world’s surface
Just as Bitcoin revolutionized currency by making it a decentralized public good, we need a decentralized and tamper-proof public memory and value layer for the world. It is governed by a foundation or DAO and runs on the Internet Computer.

## This application ...
... uses a set of canisters on the Internet Computer to create a grid of the world’s surface. Each square in the grid can hold information and value. Users authenticate with Internet Identity and interact with the squares via a React application. They read or contribute information or communicate value to other users.

Users can send value to a square’s address or contribute by adding information, which can be compensated with some of the value held by the square. When an authenticated user first visits a square, its NFT is transferred to the user’s NFT wallet. Owning a square’s NFT does not provide control over the square; it is similar to owning digital art. NFT owners are patrons of the NFT and are encouraged to improve its health metrics and gather donations. They earn part of the square’s income when its health and donation flow are good. 

The application controls the square. A decentralized governance system (e.g., a DAO, to be defined) controls the application. It sets and oversees rules for distributing value to users. The NFT transfer to users is not implemented in this MVP.

## Under the hood it ...
... creates fixed squares where each square is represented by a [geohash](https://en.wikipedia.org/wiki/Geohash). Each square / geohash is a [Dip721 Nft](https://github.com/Psychedelic/DIP721/blob/develop/spec.md) holding information such as an IPNS name pointing to metadata on IPFS. It also holds crypto addresses for Bitcoin, Ether, or USDC. The NFT points to changing information on IPFS like air quality index, crime rate, or car accident rate. This information is regularly updated via APIs or user contributions. The application interacts with Bitcoin or EVM blockchains to show balances and transact Bitcoin, Ether, or USDC between users and squares.

## Architecture

## geohash Canister

The Geohash Canister handles geolocation data, converting it into geohashes and decoding geohashes back into coordinates. It serves as the backend, managing calls to dip721_nft_container, basic_bitcoin, and basic_ethereum canisters for minting or looking up NFTs, and querying balance or metadata on IPFS. Some functionality (Ethereum balance, IPFS interaction) is not part of this MVP.

The canister provides three main functions:

- **compute_geohash**: Takes a geolocation (latitude, longitude) and returns the calculated geohash and the geographical bounds (square) it falls within. If no NFT exists, it mints a new one; otherwise, it retrieves existing NFT information.
    - **Input**: Geolocation (latitude: f64, longitude: f64)
    - **Output**: latitude and longitude boundaries, computed geohash, NFT information, Bitcoin and Ethereum balances, real-time metrics, and a flag indicating if the NFT was newly created or already existed.

- **compute_area**: Takes a geohash and decodes it back into coordinates, then calculates the geographical bounds (square) for these coordinates. If no NFT exists, it mints a new one; otherwise, it retrieves existing NFT information.
    - **Input**: Geohash (String)
    - **Output**: latitude and longitude boundaries, original geohash, NFT information, Bitcoin and Ethereum balances, real-time metrics, and a flag indicating if the NFT was newly created or already existed.

- **update_rating**: Allows users to update the rating of a square, serving as a simple MVP for user-contributed information.
    - **Input**: IPNS name (String), Rating (u32)
    - **Output**: Result indicating success or failure.
    - **What it does**: Updates the rating for the specified square, provided the rating is within the valid range (1 to 10).


### frontend canister

The frontend application is designed to interact with the Geohash Canister on the Internet Computer. It allows users to authenticate, submit geolocations or geohashes, and update ratings for specific squares. The main features include:

- **User Authentication**: Users authenticate with Internet Identity, enabling secure interaction with the application.

- **Geolocation Submission**: Users can input their geolocation (latitude and longitude) to get information about the corresponding square, including the geohash, geographical bounds, NFT information, and real-time metrics.

- **Geohash Submission**: Users can input a geohash to retrieve information about the square it represents, including geographical bounds, NFT information, and real-time metrics.

- **Update Rating**: Authenticated users can update the rating of a specific square, contributing to the user-generated data for that area.

- **Data Display**: The application displays detailed information about the square, including NFT metadata, Bitcoin and Ethereum addresses and balances, and real-time metrics like air quality, crime rate, and car accident rate.

- **Fetch Geolocation**: Users can fetch their current geolocation using the browser's geolocation API to simplify the submission process.

This frontend serves as the interface for interacting with the geolocation data and NFTs managed by the Geohash Canister, providing a user-friendly way to read and contribute information tied to specific squares on the world's surface.

### basic_bitcoin canister



### basic_ethereum canister

### dip721_nft_container

### geohash canister




# 2. Tech

## 2.2 Frontend Canister Logic

## Comments
Please note:
This code is unfinished / in development