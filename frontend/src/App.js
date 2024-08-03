import React, { useState, useEffect } from 'react';
import { createActor } from './declarations/geohash';
import GeolocationMap from './GeolocationMap';
import './tailwind.css';
import { AuthClient } from "@dfinity/auth-client";

function App() {
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [geohash, setGeohash] = useState('');
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);
  const [location, setLocation] = useState(null);
  const [bounds, setBounds] = useState(null);
  const [mapGeohash, setMapGeohash] = useState('');
  const [isUserLocation, setIsUserLocation] = useState(false); 
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [authClient, setAuthClient] = useState(null);
  const [rating, setRating] = useState('');
  const [showUpdateRating, setShowUpdateRating] = useState(false);

  useEffect(() => {
    const initAuthClient = async () => {
      const client = await AuthClient.create();
      setAuthClient(client);
      setIsAuthenticated(await client.isAuthenticated());
    };

    initAuthClient();
  }, []);

  const login = async () => {
    await authClient.login({
      identityProvider: `http://${process.env.REACT_APP_INTERNET_IDENTITY_CANISTER_ID}.localhost:8001/`,
      onSuccess: () => {
        setIsAuthenticated(true);
      },
    });
  };

  const logout = async () => {
    await authClient.logout();
    setIsAuthenticated(false);
  };

  console.log("Geohash Canister ID:", process.env.REACT_APP_GEOHASH_CANISTER_ID);

  const geohashActor = createActor(process.env.REACT_APP_GEOHASH_CANISTER_ID, {
    agentOptions: { host: 'http://127.0.0.1:8001' },
  });

  const handleFetchGeolocation = () => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setLatitude(position.coords.latitude);
          setLongitude(position.coords.longitude);
        },
        (error) => {
          setError('Error fetching geolocation: ' + error.message);
        }
      );
    } else {
      setError('Geolocation is not supported by this browser.');
    }
  };



  // User inputs geolocation and clicks submit to get the square information
  const handleGeolocationSubmit = async () => {
    try {
      const geolocation = { latitude: parseFloat(latitude), longitude: parseFloat(longitude) };
      console.log('Sending geolocation:', geolocation);

      const resultString = await geohashActor.compute_geohash(geolocation);
      console.log('Received result:', resultString);

      // Parse the result string to JSON
      const result = JSON.parse(resultString);

      // Enhanced logging for the entire result
      console.log('Result (full):', JSON.stringify(result, null, 2));

      // Log specific fields within the result object
      console.log('Result lat_start:', result.lat_start);
      console.log('Result lon_start:', result.lon_start);
      console.log('Result lat_end:', result.lat_end);
      console.log('Result lon_end:', result.lon_end);
      console.log('Result geohash:', result.geohash);
      console.log('Result nft_square:', JSON.stringify(result.nft_square, null, 2));
      console.log('Result created:', result.created);

      // Log the types of each field
      console.log('Type of lat_start:', typeof result.lat_start);
      console.log('Type of lon_start:', typeof result.lon_start);
      console.log('Type of lat_end:', typeof result.lat_end);
      console.log('Type of lon_end:', typeof result.lon_end);
      console.log('Type of geohash:', typeof result.geohash);
      console.log('Type of nft_square:', typeof result.nft_square);
      console.log('Type of created:', typeof result.created);



      if (result && result.lat_start !== undefined && result.lon_start !== undefined && result.lat_end !== undefined && result.lon_end !== undefined && result.geohash) {
        
        // Signals that rating button should be shown, this does only make sense when user first sees current rating, therefore he needs to input geolocation first
        setShowUpdateRating(true);
        console.log('Parsed result correctly:', result);

        setResponse(result);

        setError(null);

        setLocation({ latitude: geolocation.latitude, longitude: geolocation.longitude });
        setBounds({
          lat_start: result.lat_start,
          lat_end: result.lat_end,
          lon_start: result.lon_start,
          lon_end: result.lon_end,
        });
        setMapGeohash(result.geohash);
        setIsUserLocation(true); // Set flag to true when geolocation is submitted
      } else {
        throw new Error('Unexpected response format');
      }
    } catch (err) {
      console.error('Error:', err.message, err);
      setError(err.message);
      setResponse(null);
    }
  };

  // User inputs geohash and clicks submit to get the square information
  const handleGeohashSubmit = async () => {
    try {
      
      const resultString = await geohashActor.compute_area(geohash);
      console.log('Received result:', resultString);
  
      // Parse the result string to JSON
      const result = JSON.parse(resultString);
  
      // Enhanced logging for the entire result
      console.log('Result (full):', JSON.stringify(result, null, 2));
  
      // Log specific fields within the result object
      console.log('Result lat_start:', result.lat_start);
      console.log('Result lon_start:', result.lon_start);
      console.log('Result lat_end:', result.lat_end);
      console.log('Result lon_end:', result.lon_end);
      console.log('Result geohash:', result.geohash);
      console.log('Result nft_square:', JSON.stringify(result.nft_square, null, 2));
      console.log('Result created:', result.created);
  
      if (result && result.lat_start !== undefined && result.lon_start !== undefined && result.lat_end !== undefined && result.lon_end !== undefined && result.geohash) {
        // Signals that rating button should be shown
        setShowUpdateRating(true);
        console.log('Parsed result correctly:', result);
  
        setResponse(result);

        setError(null);
  
        setLocation({ latitude: result.lat_start, longitude: result.lon_start });
        setBounds({
          lat_start: result.lat_start,
          lat_end: result.lat_end,
          lon_start: result.lon_start,
          lon_end: result.lon_end,
        });
        setMapGeohash(result.geohash);
        setIsUserLocation(false); // Set flag to false when geohash is used
      } else {
        throw new Error('Unexpected response format');
      }
    } catch (err) {
      console.error('Error:', err.message, err);
      setError(err.message);
      setResponse(null);
    }
  };


  // Update inputs rating and clicks submit to update the rating of the square



  const handleUpdateRating = async () => {
    const ratingValue = parseInt(rating);
    if (isNaN(ratingValue) || ratingValue < 1 || ratingValue > 10) {
      setError('Rating must be a number between 1 and 10');
      return;
    }
  
    try {
      const ipnsName = response.nft_square.metadata[0].key_val_data.find(kv => kv.key === 'ipns_id').val.TextContent;
      await geohashActor.update_rating(ipnsName, ratingValue);
      console.log('Rating updated successfully');
  
      // Clear the error message
      setError('');
  
      // Update the rating in the response state
      const updatedResponse = { ...response };
      updatedResponse.real_time_metrics["Rating"] = ratingValue;
      setResponse(updatedResponse);
  
    } catch (err) {
      console.error('Error updating rating:', err.message, err);
      setError(err.message);
    }
  };

  
  return (
    <>
      {/* Styles */}
      <style>
        {`
          .aligned-table th, .aligned-table td {
            width: 25%; /* Adjust width as needed */
          }
          .aligned-table {
            table-layout: fixed;
          }
        `}
      </style>


      {/* Top navigation bar */}
      <div className="bg-indigo-600 text-white py-4">
        <div className="container mx-auto px-4 flex justify-between items-center">
          {/* Button to reset the view to default */}
          <button onClick={() => window.location.reload()} className="text-xl font-semibold hover:text-gray-300">
            A datalayer of the world
          </button>
          <div>
            {isAuthenticated ? (
              <button onClick={logout} className="text-xl font-semibold hover:text-gray-300">
                Logout
              </button>
            ) : (
              <button onClick={login} className="text-xl font-semibold hover:text-gray-300">
                Login
              </button>
            )}
          </div>
        </div>
      </div>
  
      {/* Main content section */}
      <div className="isolate bg-white">
        <main>
          <div className="container mx-auto p-4">
  
            {location && bounds && (
              <div className="mb-6">
                <GeolocationMap location={location} bounds={bounds} geohash={mapGeohash} isUserLocation={isUserLocation} />
              </div>
            )}

            {response && (
              <div className="mt-10">
                {/*
                {response.created ? (
                  <p className="text-green-500 font-bold">NFT for this square created with Token ID {response.nft_square.token_id}</p>
                ) : (
                  <p className="text-blue-500 font-bold">NFT for square exists with Token ID {response.nft_square.token_id}</p>
                )}*/}
                <div className="relative overflow-x-auto">
                  {/* Table for NFT Information */}
                  <h2 className="text-xl mb-2 text-black">NFT Information of square (Token ID: {response.nft_square.token_id})</h2>
                  <table className="aligned-table w-full text-sm text-left text-gray-500">
                    <thead className="text-xs text-gray-700 uppercase bg-gray-50">
                      <tr>
                        <th scope="col" className="px-6 py-3">Dimension</th>
                        <th scope="col" className="px-6 py-3">Value</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Geohash</th>
                        <td className="px-6 py-4">{response.geohash}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">IPNS Name</th>
                        <td className="px-6 py-4">{response.nft_square.metadata[0].key_val_data.find(kv => kv.key === 'ipns_id').val.TextContent}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Bitcoin Address</th>
                        <td className="px-6 py-4">{response.nft_square.metadata[0].key_val_data.find(kv => kv.key === 'bitcoin_address').val.TextContent}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Ethereum Address</th>
                        <td className="px-6 py-4">{response.nft_square.metadata[0].key_val_data.find(kv => kv.key === 'ethereum_address').val.TextContent}</td>
                      </tr>
                    </tbody>
                  </table>
                  {/* Table for Real-Time Metrics */}
                  <h2 className="text-xl mb-2 text-black mt-6">Real-Time Metrics</h2>
                  <table className="aligned-table w-full text-sm text-left text-gray-500">
                    <thead className="text-xs text-gray-700 uppercase bg-gray-50">
                      <tr>
                        <th scope="col" className="px-6 py-3">Dimension</th>
                        <th scope="col" className="px-6 py-3">Value</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Bitcoin Balance</th>
                        <td className="px-6 py-4">{response.bitcoin_balance}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Ethereum Balance</th>
                        <td className="px-6 py-4">{response.ethereum_balance}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Air quality index</th>
                        <td className="px-6 py-4">{response.real_time_metrics["Air quality index"]}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Crime Rate</th>
                        <td className="px-6 py-4">{response.real_time_metrics["Crime rate"]}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Car Accident Rate</th>
                        <td className="px-6 py-4">{response.real_time_metrics["Car accident rate"]}</td>
                      </tr>
                      <tr className="odd:bg-white even:bg-gray-50 border-b">
                        <th scope="row" className="px-6 py-4 font-medium text-gray-900">Rating</th>
                        <td className="px-6 py-4">{response.real_time_metrics.Rating}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            )}

            {isAuthenticated && showUpdateRating && (
              <div className="mb-6 mt-10">
                <h2 className="text-xl mb-2">Update Rating</h2>
                <input
                  className="border p-2 mb-2 w-full"
                  type="number"
                  placeholder="Rating (1-10)"
                  value={rating}
                  min="1"
                  max="10"
                  onChange={(e) => setRating(e.target.value)}
                />
                <button
                  className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700"
                  onClick={handleUpdateRating}
                >
                  Submit Rating
                </button>
                {error && <p className="text-red-500">{error}</p>}
              </div>
            )}

            <div className="mb-6 mt-10">
              <h2 className="text-xl mb-2">Input Geolocation</h2>
              <input
                className="border p-2 mb-2 w-full"
                type="number"
                placeholder="Latitude"
                value={latitude}
                onChange={(e) => setLatitude(e.target.value)}
              />
              <input
                className="border p-2 mb-2 w-full"
                type="number"
                placeholder="Longitude"
                value={longitude}
                onChange={(e) => setLongitude(e.target.value)}
              />
              <button
                className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700"
                onClick={handleGeolocationSubmit}
              >
                Submit Geolocation
              </button>
            </div>
  
            <div className="mb-6">
              <h2 className="text-xl mb-2">Input Geohash</h2>
              <input
                className="border p-2 mb-2 w-full"
                type="text"
                placeholder="Geohash"
                value={geohash}
                onChange={(e) => setGeohash(e.target.value)}
              />
              <button
                className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700"
                onClick={handleGeohashSubmit}
              >
                Submit Geohash
              </button>
            </div>
  
            <div className="flex justify-center items-center mt-10 space-x-4">
              <button
                className="rounded-md bg-indigo-600 px-3.5 py-1.5 text-base font-semibold leading-7 text-white shadow-sm hover:bg-indigo-500"
                onClick={handleFetchGeolocation}
              >
                Fetch Geolocation
              </button>
            </div>
  
            {error && (
              <div className="mb-6">
                <h2 className="text-xl mb-2">Error</h2>
                <pre className="bg-red-100 text-red-700 p-4 rounded">{error}</pre>
              </div>
            )}
          </div>
        </main>
      </div>
    </>
  );
}

export default App;
