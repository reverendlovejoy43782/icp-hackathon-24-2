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

export interface KeyValRecord {
  key: string;
  val: MetadataVal;
}

export interface MetadataDesc {
  purpose: string;
  key_val_data: KeyValRecord[];
  data: Uint8Array;
}

export interface Nft {
  owner: Principal;
  token_id: bigint;
  metadata: MetadataDesc;
  content: Uint8Array;
}

export interface AreaResponse {
  lat_start: number;
  lon_start: number;
  lat_end: number;
  lon_end: number;
  geohash: string;
  nft_square: Nft | null; // Optional field for the NFT
}

export interface Geolocation {
  latitude: number;
  longitude: number;
}

export interface _SERVICE {
  compute_area: ActorMethod<[string], AreaResponse>;
  compute_geohash: ActorMethod<[Geolocation], AreaResponse>;
}

export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];