// src/GeolocationMap.js
import React, { useEffect, useState } from "react";

const GeolocationMap = ({ location, bounds, geohash }) => {
  const [mapLoadError, setMapLoadError] = useState(null);
  const [isScriptLoaded, setIsScriptLoaded] = useState(false);

  const loadGoogleMapsScript = () => {
    if (window.google && window.google.maps) {
      setIsScriptLoaded(true);
      return;
    }

    if (document.querySelector('script[src^="https://maps.googleapis.com/maps/api/js"]')) {
      return;
    }

    const script = document.createElement("script");
    script.src = `https://maps.googleapis.com/maps/api/js?key=${process.env.REACT_APP_GOOGLE_MAPS_API_KEY}`;
    script.async = true;
    script.defer = true;

    script.onload = () => {
      setIsScriptLoaded(true);
    };
    script.onerror = () => {
      setMapLoadError("Failed to load Google Maps script.");
    };

    document.head.appendChild(script);
  };

  useEffect(() => {
    loadGoogleMapsScript();
  }, []);

  useEffect(() => {
    if (isScriptLoaded && window.google && location?.latitude && location?.longitude) {
      const mapInstance = new window.google.maps.Map(document.getElementById("map"), {
        center: { lat: location.latitude, lng: location.longitude },
        zoom: 14,
      });

      new window.google.maps.Marker({
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

        new window.google.maps.Marker({
          position: { lat: offsetLat, lng: centerLng },
          map: mapInstance,
          label: {
            text: geohash,
            color: "#FF0000",
          },
          icon: {
            path: window.google.maps.SymbolPath.CIRCLE,
            fillOpacity: 0,
            strokeOpacity: 0,
            scale: 0
          }
        });
      }
    }
  }, [location, bounds, isScriptLoaded, geohash]);

  if (mapLoadError) {
    return <div>Error loading Google Maps: {mapLoadError}</div>;
  }

  return (
    <div id="map" style={{ width: "100%", height: "300px" }}>
      {/* Map will be rendered here */}
    </div>
  );
};

export default GeolocationMap;




/*
import React, { useEffect, useState } from "react";

const GeolocationMap = ({ location, bounds, geohash }) => {
  const [mapLoadError, setMapLoadError] = useState(null);
  const [isScriptLoaded, setIsScriptLoaded] = useState(false);

  const loadGoogleMapsScript = () => {
    if (window.google && window.google.maps) {
      setIsScriptLoaded(true);
      return;
    }

    if (document.querySelector('script[src^="https://maps.googleapis.com/maps/api/js"]')) {
      return;
    }

    const script = document.createElement("script");
    script.src = `https://maps.googleapis.com/maps/api/js?key=${process.env.REACT_APP_GOOGLE_MAPS_API_KEY}`;
    script.async = true;
    script.defer = true;

    script.onload = () => {
      setIsScriptLoaded(true);
    };
    script.onerror = () => {
      setMapLoadError("Failed to load Google Maps script.");
    };

    document.head.appendChild(script);
  };

  useEffect(() => {
    loadGoogleMapsScript();
  }, []);

  useEffect(() => {
    if (isScriptLoaded && window.google && location?.latitude && location?.longitude) {
      const mapInstance = new window.google.maps.Map(document.getElementById("map"), {
        center: { lat: location.latitude, lng: location.longitude },
        zoom: 14,
      });

      new window.google.maps.Marker({
        position: { lat: location.latitude, lng: location.longitude },
        map: mapInstance,
        title: "Your Location",
      });

      if (bounds) {
        new window.google.maps.Rectangle({
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



        const centerLng = (bounds.lon_start + bounds.lon_end) / 2;
        const offsetLat = bounds.lat_end - (bounds.lat_end - bounds.lat_start) * 0.09;

        new window.google.maps.Marker({
          position: { lat: offsetLat, lng: centerLng },
          map: mapInstance,
          label: {
            text: geohash,
            color: "#FF0000",
          },
          icon: {
            path: window.google.maps.SymbolPath.CIRCLE,
            fillOpacity: 0,
            strokeOpacity: 0,
            scale: 0
          }
        });
      }
    }
  }, [location, bounds, isScriptLoaded, geohash]);

  if (mapLoadError) {
    return <div>Error loading Google Maps: {mapLoadError}</div>;
  }

  return (
    <div id="map" style={{ width: "100%", height: "300px" }}>
    </div>
  );
};

export default GeolocationMap;
*/