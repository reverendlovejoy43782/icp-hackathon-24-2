export const idlFactory = ({ IDL }) => {
  const Geolocation = IDL.Record({
    'latitude' : IDL.Float64,
    'longitude' : IDL.Float64,
  });
  return IDL.Service({
    'compute_geohash' : IDL.Func([Geolocation], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
