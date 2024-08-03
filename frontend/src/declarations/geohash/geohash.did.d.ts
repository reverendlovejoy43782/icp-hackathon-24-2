import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface MetadataVal {
  TextContent?: string;
  BlobContent?: Uint8Array;
  NatContent?: bigint;
  Nat8Content?: number;
  Nat16Content?: number;
  Nat32Content?: number;
  Nat64Content?: bigint;
}

export interface MetadataKeyVal {
  key: string;
  val: MetadataVal;
}

export interface MetadataPart {
  purpose: string;
  key_val_data: MetadataKeyVal[];
  data: Uint8Array;
}

export interface MetadataLookupPart {
  purpose: string;
  key_val_data: MetadataKeyVal[];
  data: Uint8Array;
}

export type MetadataLookupDesc = MetadataLookupPart[];

export interface Nft {
  owner: Principal;
  token_id: bigint;
  metadata: MetadataLookupDesc;
  content: Uint8Array;
}

export interface Wallet {
  ether: string;
  bitcoin: string;
}


export interface SquareProperties {
  geohash: string;
  metadata: string;
  wallet: Wallet;
}

export interface Geolocation {
  latitude: number;
  longitude: number;
}

export interface AreaResponse {
  lat_start: number;
  lon_start: number;
  lat_end: number;
  lon_end: number;
  geohash: string;
  nft_square: Nft | null; // Optional field for the NFT
  created: boolean; // Indicating if NFT was created
}



export interface _SERVICE {
  compute_area: ActorMethod<[string], string>;
  //compute_area: ActorMethod<[string], AreaResponse>;
  compute_geohash: ActorMethod<[Geolocation], string>;
  //compute_geohash: ActorMethod<[Geolocation], AreaResponse>;
  update_rating: ActorMethod<[string, number], Result>;
}