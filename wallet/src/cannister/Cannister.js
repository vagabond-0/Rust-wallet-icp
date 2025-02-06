import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "./contract_backend.did.js";

// Local canister ID
const canisterId = process.env.NEXT_PUBLIC_CANISTER_ID || "by6od-j4aaa-aaaaa-qaadq-cai";

// Local host URL for development
const host = process.env.NEXT_PUBLIC_IC_HOST || "http://127.0.0.1:4943";

// Initialize the agent
export const initializeAgent = async () => {
  const agent = new HttpAgent({
    host: host,
  });

  // Only for local development - skip when deploying to IC mainnet
  if (host === "http://127.0.0.1:4943") {
    await agent.fetchRootKey();
  }

  return agent;
};

// Initialize the canister
export const initializeCanister = async () => {
  const agent = await initializeAgent();
  
  // Create an actor with the canister interface
  const canister = Actor.createActor(idlFactory, {
    agent,
    canisterId: canisterId,
  });

  return canister;
};