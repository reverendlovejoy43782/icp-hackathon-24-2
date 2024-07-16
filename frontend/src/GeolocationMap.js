import React, { useEffect, useState, useRef } from "react";
import { Loader } from "@googlemaps/js-api-loader";

const GeolocationMap = ({ location, bounds, geohash, isUserLocation }) => {
  const [mapLoadError, setMapLoadError] = useState(null);
  const mapRef = useRef(null); // Create a ref for the map div

  useEffect(() => {
    const loader = new Loader({
      apiKey: process.env.REACT_APP_GOOGLE_MAPS_API_KEY,
      version: "weekly",
      libraries: ["marker"], // Load the marker library
    });

    const loadMap = async () => {
      try {
        await loader.load();
        if (mapRef.current && (location?.latitude && location?.longitude || bounds)) {
          const { AdvancedMarkerElement } = await window.google.maps.importLibrary("marker");

          const mapInstance = new window.google.maps.Map(mapRef.current, {
            center: location ? { lat: location.latitude, lng: location.longitude } : { lat: (bounds.lat_start + bounds.lat_end) / 2, lng: (bounds.lon_start + bounds.lon_end) / 2 },
            zoom: 14,
            mapId: "MY_MAP_ID", 
          });

          // Set marker only if location is provided and it's user location
          if (location && isUserLocation) {
            const markerElement = document.createElement('div');
            markerElement.textContent = "Your Location";
            new AdvancedMarkerElement({
              position: { lat: location.latitude, lng: location.longitude },
              map: mapInstance,
              title: "Your Location", // Tooltip for the marker
            });
          }

          if (bounds) {
            const rectangle = new window.google.maps.Rectangle({
              strokeColor: "#FF0000",
              strokeOpacity: 0.1,
              strokeWeight: 1,
              fillColor: "#FF0000",
              fillOpacity: 0.2,
              map: mapInstance,
              bounds: {
                north: bounds.lat_end,
                south: bounds.lat_start,
                east: bounds.lon_end,
                west: bounds.lon_start,
              },
            });

            // Fit map to the bounds of the rectangle
            mapInstance.fitBounds(rectangle.getBounds());

            const centerLat = (bounds.lat_start + bounds.lat_end) / 2;
            const centerLng = (bounds.lon_start + bounds.lon_end) / 2;

            const geohashMarkerElement = document.createElement('div');
            geohashMarkerElement.textContent = geohash;
            geohashMarkerElement.style.color = "#000000";
            geohashMarkerElement.style.fontWeight = "bold";
            geohashMarkerElement.style.fontSize = "16px";

            new AdvancedMarkerElement({
              position: { lat: centerLat, lng: centerLng },
              map: mapInstance,
              title: geohash, // Tooltip for the geohash marker
              content: geohashMarkerElement,
            });
          }
        }
      } catch (error) {
        setMapLoadError("Failed to load Google Maps: " + error.message);
      }
    };

    loadMap();
  }, [location, bounds, geohash, isUserLocation]);

  if (mapLoadError) {
    return <div>Error loading Google Maps: {mapLoadError}</div>;
  }

  return (
    <div id="map" ref={mapRef} style={{ width: "100%", height: "300px" }}>
      {/* Map will be rendered here */}
    </div>
  );
};

export default GeolocationMap;




/*
import React, { useEffect, useState, useRef } from "react";
import { Loader } from "@googlemaps/js-api-loader";

const GeolocationMap = ({ location, bounds, geohash, isUserLocation }) => {
  const [mapLoadError, setMapLoadError] = useState(null);
  const mapRef = useRef(null); // Create a ref for the map div

  useEffect(() => {
    const loader = new Loader({
      apiKey: process.env.REACT_APP_GOOGLE_MAPS_API_KEY,
      version: "weekly",
      libraries: ["marker"], // Load the marker library
    });

    const loadMap = async () => {
      try {
        await loader.load();
        if (mapRef.current && (location?.latitude && location?.longitude || bounds)) {
          const { AdvancedMarkerElement } = await window.google.maps.importLibrary("marker");

          const mapInstance = new window.google.maps.Map(mapRef.current, {
            center: location ? { lat: location.latitude, lng: location.longitude } : { lat: (bounds.lat_start + bounds.lat_end) / 2, lng: (bounds.lon_start + bounds.lon_end) / 2 },
            zoom: 14,
            mapId: "YOUR_MAP_ID", 
          });

          // Set marker only if location is provided and it's user location
          if (location && isUserLocation) {
            const markerElement = document.createElement('div');
            markerElement.textContent = "Your Location";
            new AdvancedMarkerElement({
              position: { lat: location.latitude, lng: location.longitude },
              map: mapInstance,
              title: "Your Location", // Tooltip for the marker
            });
          }

          if (bounds) {
            const rectangle = new window.google.maps.Rectangle({
              strokeColor: "#FF0000",
              strokeOpacity: 0.1,
              strokeWeight: 1,
              fillColor: "#FF0000",
              fillOpacity: 0.2,
              map: mapInstance,
              bounds: {
                north: bounds.lat_end,
                south: bounds.lat_start,
                east: bounds.lon_end,
                west: bounds.lon_start,
              },
            });

            // Fit map to the bounds of the rectangle
            mapInstance.fitBounds(rectangle.getBounds());

            const centerLat = (bounds.lat_start + bounds.lat_end) / 2;
            const centerLng = (bounds.lon_start + bounds.lon_end) / 2;

            const geohashMarkerElement = document.createElement('div');
            geohashMarkerElement.textContent = geohash;
            geohashMarkerElement.style.color = "#FF0000";

            new AdvancedMarkerElement({
              position: { lat: centerLat, lng: centerLng },
              map: mapInstance,
              title: geohash, // Tooltip for the geohash marker
              content: geohashMarkerElement,
            });
          }
        }
      } catch (error) {
        setMapLoadError("Failed to load Google Maps: " + error.message);
      }
    };

    loadMap();
  }, [location, bounds, geohash, isUserLocation]);

  if (mapLoadError) {
    return <div>Error loading Google Maps: {mapLoadError}</div>;
  }

  return (
    <div id="map" ref={mapRef} style={{ width: "100%", height: "300px" }}>
      { Map will be rendered here }
    </div>
  );
};

export default GeolocationMap;



###

import React, { useEffect, useState, useRef } from "react";
import { Loader } from "@googlemaps/js-api-loader";

const GeolocationMap = ({ location, bounds, geohash }) => {
  const [mapLoadError, setMapLoadError] = useState(null);
  const mapRef = useRef(null); // Create a ref for the map div

  useEffect(() => {
    const loader = new Loader({
      apiKey: process.env.REACT_APP_GOOGLE_MAPS_API_KEY,
      version: "weekly",
      libraries: ["marker"], // Load the marker library
    });

    const loadMap = async () => {
      try {
        await loader.load();
        if (mapRef.current && location?.latitude && location?.longitude) {
          const { AdvancedMarkerElement } = await window.google.maps.importLibrary("marker");

          const mapInstance = new window.google.maps.Map(mapRef.current, {
            center: { lat: location.latitude, lng: location.longitude },
            zoom: 14,
            mapId: "YOUR_MAP_ID", // Replace with your actual Map ID
          });

          // Using google.maps.marker.AdvancedMarkerElement instead of google.maps.Marker
          const markerElement = document.createElement('div');
          markerElement.textContent = "Your Location";
          const marker = new AdvancedMarkerElement({
            position: { lat: location.latitude, lng: location.longitude },
            map: mapInstance,
            title: "Your Location",
          });

          if (bounds) {
            const rectangle = new window.google.maps.Rectangle({
              strokeColor: "#FF0000",
              strokeOpacity: 0.1,
              strokeWeight: 1,
              fillColor: "#FF0000",
              fillOpacity: 0.2,
              map: mapInstance,
              bounds: {
                north: bounds.lat_end,
                south: bounds.lat_start,
                east: bounds.lon_end,
                west: bounds.lon_start,
              },
            });

            // Fit map to the bounds of the rectangle
            mapInstance.fitBounds(rectangle.getBounds());

            const centerLng = (bounds.lon_start + bounds.lon_end) / 2;
            const offsetLat = bounds.lat_end - (bounds.lat_end - bounds.lat_start) * 0.09;

            const geohashMarkerElement = document.createElement('div');
            geohashMarkerElement.textContent = geohash;
            geohashMarkerElement.style.color = "#FF0000";

            new AdvancedMarkerElement({
              position: { lat: offsetLat, lng: centerLng },
              map: mapInstance,
              title: geohash,
              content: geohashMarkerElement,
            });
          }
        }
      } catch (error) {
        setMapLoadError("Failed to load Google Maps: " + error.message);
      }
    };

    loadMap();
  }, [location, bounds, geohash]);

  if (mapLoadError) {
    return <div>Error loading Google Maps: {mapLoadError}</div>;
  }

  return (
    <div id="map" ref={mapRef} style={{ width: "100%", height: "300px" }}>
      { Map will be rendered here }
    </div>
  );
};

export default GeolocationMap;
*/