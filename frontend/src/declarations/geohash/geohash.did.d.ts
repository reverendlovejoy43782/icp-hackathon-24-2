import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AreaResponse {
  lat_start: number;
  lon_start: number;
  lat_end: number;
  lon_end: number;
  geohash: string;
}

export interface Geolocation {
  latitude: number;
  longitude: number;
}

export interface _SERVICE {
  query_compute_area: ActorMethod<[string], AreaResponse>;
  query_compute_geohash: ActorMethod<[Geolocation], AreaResponse>;
}

export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
