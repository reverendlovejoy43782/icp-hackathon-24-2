const fs = require("fs");
const path = require("path");

function initCanisterEnv() {
  let localCanisters, prodCanisters;
  try {
    localCanisters = require(path.resolve(".dfx", "local", "canister_ids.json"));
  } catch (error) {
    console.log("No local canister_ids.json found");
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found");
  }

  const network = process.env.DFX_NETWORK || (process.env.NODE_ENV === "production" ? "ic" : "local");
  const canisterConfig = network === "local" ? localCanisters : prodCanisters;

  const envVars = canisterConfig
    ? Object.entries(canisterConfig).reduce((prev, current) => {
        const [canisterName, canisterDetails] = current;
        const canisterId = canisterDetails[network];
        if (canisterName === "geohash") {
          prev["REACT_APP_" + canisterName.toUpperCase() + "_CANISTER_ID"] = canisterId;
        }
        if (canisterName === "internet_identity") {
          prev["REACT_APP_INTERNET_IDENTITY_CANISTER_ID"] = canisterId;
        }
        return prev;
      }, {})
    : {};

  return envVars;
}

function readEnvFile(filePath) {
  if (!fs.existsSync(filePath)) {
    return {};
  }
  const envContent = fs.readFileSync(filePath, "utf8");
  return envContent.split("\n").reduce((acc, line) => {
    const [key, value] = line.split("=");
    if (key) {
      acc[key.trim()] = value.trim();
    }
    return acc;
  }, {});
}

function writeEnvFile(filePath, envVars) {
  const existingVars = readEnvFile(filePath);
  const updatedVars = { ...existingVars, ...envVars };
  const envContent = Object.entries(updatedVars)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");
  fs.writeFileSync(filePath, envContent);
}

const envVars = initCanisterEnv();
const frontendEnvVars = {
  "REACT_APP_GEOHASH_CANISTER_ID": envVars["REACT_APP_GEOHASH_CANISTER_ID"],
  "REACT_APP_INTERNET_IDENTITY_CANISTER_ID": envVars["REACT_APP_INTERNET_IDENTITY_CANISTER_ID"]
};

// Write to .env file in root/frontend
writeEnvFile(path.resolve(__dirname, "frontend", ".env"), frontendEnvVars);

console.log("Environment variables set in frontend/.env:");
console.log(frontendEnvVars);