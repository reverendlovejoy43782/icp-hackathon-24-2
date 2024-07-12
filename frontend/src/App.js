import React, { useState } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as geohash_idl, canisterId as geohash_id } from './declarations/geohash';

function App() {
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [geohash, setGeohash] = useState('');
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);

  // Configure the agent to use the correct host for the geohash canister
  const agent = new HttpAgent({ host: 'http://127.0.0.1:8001' });
  const geohashActor = Actor.createActor(geohash_idl, { agent, canisterId: geohash_id });

  const handleGeolocationSubmit = async () => {
    try {
      const geolocation = { latitude: parseFloat(latitude), longitude: parseFloat(longitude) };
      const result = await geohashActor.query_compute_geohash(geolocation);
      setResponse(result);
      setError(null);
    } catch (err) {
      setError(err.message);
      setResponse(null);
    }
  };

  const handleGeohashSubmit = async () => {
    try {
      const result = await geohashActor.query_compute_area(geohash);
      setResponse(result);
      setError(null);
    } catch (err) {
      setError(err.message);
      setResponse(null);
    }
  };

  return (
    <div>
      <h1>Geohash Frontend</h1>
      <div>
        <h2>Input Geolocation</h2>
        <input
          type="number"
          placeholder="Latitude"
          value={latitude}
          onChange={(e) => setLatitude(e.target.value)}
        />
        <input
          type="number"
          placeholder="Longitude"
          value={longitude}
          onChange={(e) => setLongitude(e.target.value)}
        />
        <button onClick={handleGeolocationSubmit}>Submit Geolocation</button>
      </div>
      <div>
        <h2>Input Geohash</h2>
        <input
          type="text"
          placeholder="Geohash"
          value={geohash}
          onChange={(e) => setGeohash(e.target.value)}
        />
        <button onClick={handleGeohashSubmit}>Submit Geohash</button>
      </div>
      {response && (
        <div>
          <h2>Response</h2>
          <pre>{JSON.stringify(response, null, 2)}</pre>
        </div>
      )}
      {error && (
        <div>
          <h2>Error</h2>
          <pre>{error}</pre>
        </div>
      )}
    </div>
  );
}

export default App;