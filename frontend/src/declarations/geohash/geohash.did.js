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

  const MetadataKeyVal = IDL.Record({
    key: IDL.Text,
    val: MetadataVal,
  });

  const MetadataPart = IDL.Record({
    purpose: IDL.Variant({ Preview: IDL.Null, Rendered: IDL.Null }),
    key_val_data: IDL.Vec(MetadataKeyVal),
    data: IDL.Vec(IDL.Nat8),
  });

  const MetadataLookupPart = IDL.Record({
    purpose: IDL.Variant({ Preview: IDL.Null, Rendered: IDL.Null }),
    key_val_data: IDL.Vec(MetadataKeyVal),
    data: IDL.Vec(IDL.Nat8),
  });

  const MetadataLookupDesc = IDL.Vec(MetadataLookupPart);

  const Nft = IDL.Record({
    owner: IDL.Principal,
    token_id: IDL.Nat64,
    metadata: MetadataLookupDesc,
    content: IDL.Vec(IDL.Nat8),
  });

  const Wallet = IDL.Record({
    bitcoin: IDL.Text,
    ether: IDL.Text,
  });

  const SquareProperties = IDL.Record({
    geohash: IDL.Text,
    metadata: IDL.Text,
    wallet: Wallet,
  });

  const Geolocation = IDL.Record({
    latitude: IDL.Float64,
    longitude: IDL.Float64,
  });

  const AreaResponse = IDL.Record({
    lat_start: IDL.Float64,
    lon_start: IDL.Float64,
    lat_end: IDL.Float64,
    lon_end: IDL.Float64,
    geohash: IDL.Text,
    nft_square: IDL.Opt(Nft),
    created: IDL.Bool,
  });


  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  
  return IDL.Service({
    compute_area: IDL.Func([IDL.Text], [IDL.Text], []),
    //compute_area: IDL.Func([IDL.Text], [AreaResponse], []), // Uncomment if returning AreaResponse
    compute_geohash: IDL.Func([Geolocation], [IDL.Text], []),
    //compute_geohash: IDL.Func([Geolocation], [AreaResponse], []), // Uncomment if returning AreaResponse
    update_rating: IDL.Func([IDL.Text, IDL.Nat32], [Result], []),
  });
};

export const init = ({ IDL }) => {
  return [];
};