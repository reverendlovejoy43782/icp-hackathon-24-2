export const idlFactory = ({ IDL }) => {
  const MetadataVal = IDL.Variant({
    TextContent: IDL.Text,
    BlobContent: IDL.Vec(IDL.Nat8),
    NatContent: IDL.Nat,
    Nat8Content: IDL.Nat8,
    Nat16Content: IDL.Nat16,
    Nat32Content: IDL.Nat32,
    Nat64Content: IDL.Nat64,
  });

  const KeyValRecord = IDL.Record({
    key: IDL.Text,
    val: MetadataVal,
  });

  const MetadataDesc = IDL.Record({
    purpose: IDL.Text,
    key_val_data: IDL.Vec(KeyValRecord),
    data: IDL.Vec(IDL.Nat8),
  });

  const Nft = IDL.Record({
    owner: IDL.Principal,
    token_id: IDL.Nat64,
    metadata: MetadataDesc,
    content: IDL.Vec(IDL.Nat8),
  });

  const AreaResponse = IDL.Record({
    lat_start: IDL.Float64,
    lon_start: IDL.Float64,
    lat_end: IDL.Float64,
    lon_end: IDL.Float64,
    geohash: IDL.Text,
    nft_square: IDL.Opt(Nft),
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