import React, { useState } from 'react';
import { createActor } from './declarations/geohash';
import GeolocationMap from './GeolocationMap';

function App() {
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [geohash, setGeohash] = useState('');
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);
  const [location, setLocation] = useState(null);
  const [bounds, setBounds] = useState(null);
  const [mapGeohash, setMapGeohash] = useState('');

  const geohashActor = createActor(process.env.REACT_APP_GEOHASH_CANISTER_ID, {
    agentOptions: { host: 'http://127.0.0.1:8001' },
  });

  const handleGeolocationSubmit = async () => {
    try {
      const geolocation = { latitude: parseFloat(latitude), longitude: parseFloat(longitude) };
      const result = await geohashActor.query_compute_geohash(geolocation);
      setResponse(result);
      setError(null);

      setLocation({ latitude: geolocation.latitude, longitude: geolocation.longitude });
      setBounds({
        lat_start: result[0].lat_start,
        lat_end: result[0].lat_end,
        lon_start: result[0].lon_start,
        lon_end: result[0].lon_end,
      });
      setMapGeohash(result[1]);
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

      setLocation({ latitude: result.lat_start, longitude: result.lon_start });
      setBounds({
        lat_start: result.lat_start,
        lat_end: result.lat_end,
        lon_start: result.lon_start,
        lon_end: result.lon_end,
      });
      setMapGeohash(result.geohash);
    } catch (err) {
      setError(err.message);
      setResponse(null);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Geohash Frontend</h1>
      <div className="mb-6">
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
      {response && (
        <div className="mb-6">
          <h2 className="text-xl mb-2">Response</h2>
          <pre className="bg-gray-100 p-4 rounded">{JSON.stringify(response, null, 2)}</pre>
        </div>
      )}
      {error && (
        <div className="mb-6">
          <h2 className="text-xl mb-2">Error</h2>
          <pre className="bg-red-100 text-red-700 p-4 rounded">{error}</pre>
        </div>
      )}
      {location && bounds && (
        <div className="mb-6">
          <GeolocationMap location={location} bounds={bounds} geohash={mapGeohash} />
        </div>
      )}
    </div>
  );
}

export default App;




/*
import React, { useState } from 'react';
import { createActor } from './declarations/geohash';
import GeolocationMap from './GeolocationMap';

function App() {
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [geohash, setGeohash] = useState('');
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);
  const [location, setLocation] = useState(null);
  const [bounds, setBounds] = useState(null);
  const [mapGeohash, setMapGeohash] = useState('');

  const geohashActor = createActor(process.env.REACT_APP_GEOHASH_CANISTER_ID, {
    agentOptions: { host: 'http://127.0.0.1:8001' },
  });

  const handleGeolocationSubmit = async () => {
    try {
      const geolocation = { latitude: parseFloat(latitude), longitude: parseFloat(longitude) };
      const result = await geohashActor.query_compute_geohash(geolocation);
      setResponse(result);
      setError(null);

      // Update map location and bounds
      setLocation({ latitude: geolocation.latitude, longitude: geolocation.longitude });
      setBounds({
        lat_start: result[0].lat_start,
        lat_end: result[0].lat_end,
        lon_start: result[0].lon_start,
        lon_end: result[0].lon_end,
      });
      setMapGeohash(result[1]);
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

      // Update map location and bounds
      setLocation({ latitude: result.lat_start, longitude: result.lon_start });
      setBounds({
        lat_start: result.lat_start,
        lat_end: result.lat_end,
        lon_start: result.lon_start,
        lon_end: result.lon_end,
      });
      setMapGeohash(result.geohash);
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
      {location && bounds && (
        <GeolocationMap location={location} bounds={bounds} geohash={mapGeohash} />
      )}
    </div>
  );
}

export default App;




###



import React, { useState } from 'react';
import { createActor } from './declarations/geohash';

function App() {
  const [latitude, setLatitude] = useState('');
  const [longitude, setLongitude] = useState('');
  const [geohash, setGeohash] = useState('');
  const [response, setResponse] = useState(null);
  const [error, setError] = useState(null);

  // Configure the actor for the geohash canister
  const geohashActor = createActor(process.env.REACT_APP_GEOHASH_CANISTER_ID, {
    agentOptions: { host: 'http://127.0.0.1:8001' },
  });

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
*/