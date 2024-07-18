export const idlFactory = ({ IDL }) => {
  const Nft = IDL.Record({
    canister: IDL.Principal,
    index: IDL.Nat64,
    metadata: IDL.Text,
  });

  const AreaResponse = IDL.Record({
    lat_start: IDL.Float64,
    lon_start: IDL.Float64,
    lat_end: IDL.Float64,
    lon_end: IDL.Float64,
    geohash: IDL.Text,
    owned_nfts: IDL.Vec(Nft),
  });

  const Geolocation = IDL.Record({
    latitude: IDL.Float64,
    longitude: IDL.Float64,
  });

  return IDL.Service({
    compute_area: IDL.Func([IDL.Text], [AreaResponse], []),
    compute_geohash: IDL.Func([Geolocation], [AreaResponse], []),
  });
};

export const init = ({ IDL }) => {
  return [];
};