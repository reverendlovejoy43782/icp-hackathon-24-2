export const idlFactory = ({ IDL }) => {
  const AreaResponse = IDL.Record({
    'lat_start' : IDL.Float64,
    'lon_start' : IDL.Float64,
    'geohash' : IDL.Text,
    'lat_end' : IDL.Float64,
    'lon_end' : IDL.Float64,
  });
  const Geolocation = IDL.Record({
    'latitude' : IDL.Float64,
    'longitude' : IDL.Float64,
  });
  return IDL.Service({
    'query_compute_area' : IDL.Func([IDL.Text], [AreaResponse], ['query']),
    'query_compute_geohash' : IDL.Func(
        [Geolocation],
        [AreaResponse],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
