import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "./geohash.did.js";

export { idlFactory } from "./geohash.did.js";

export const canisterId = process.env.REACT_APP_GEOHASH_CANISTER_ID;

export const createActor = (canisterId, options = {}) => {
  const agent = options.agent || new HttpAgent({ ...options.agentOptions });

  if (options.agent && options.agentOptions) {
    console.warn(
      "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent."
    );
  }

  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  // Enhanced actor creation with logging
  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options.actorOptions,
  });

  const originalComputeGeohash = actor.compute_geohash;
  actor.compute_geohash = async (...args) => {
    console.log('compute_geohash called with:', ...args);
    const result = await originalComputeGeohash(...args);
    console.log('compute_geohash result:', result);
    return result;
  };

  const originalComputeArea = actor.compute_area;
  actor.compute_area = async (...args) => {
    console.log('compute_area called with:', ...args);
    const result = await originalComputeArea(...args);
    console.log('compute_area result:', result);
    return result;
  };

  return actor;
};

export const geohash = canisterId ? createActor(canisterId) : undefined;


/*
import { Actor, HttpAgent } from "@dfinity/agent";

// Imports and re-exports candid interface
import { idlFactory } from "./geohash.did.js";
export { idlFactory } from "./geohash.did.js";

/* CANISTER_ID is replaced by webpack based on node environment
 * Note: canister environment variable will be standardized as
 * process.env.CANISTER_ID_<CANISTER_NAME_UPPERCASE>
 * beginning in dfx 0.15.0
 
export const canisterId = process.env.REACT_APP_GEOHASH_CANISTER_ID;

export const createActor = (canisterId, options = {}) => {
  const agent = options.agent || new HttpAgent({ ...options.agentOptions });

  if (options.agent && options.agentOptions) {
    console.warn(
      "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent."
    );
  }

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options.actorOptions,
  });
};

export const geohash = canisterId ? createActor(canisterId) : undefined;
*/