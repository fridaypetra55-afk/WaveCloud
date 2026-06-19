#![no_std]

use soroban_sdk::{contractimpl, symbol, Address, Env, Symbol, Vec, BytesN};

pub struct WaveCloudContract;

const POLICY_COUNT: &str = "POLICY_COUNT";
const ORACLE_SIGNER: &str = "ORACLE_SIGNER";
const POOL_BALANCE: &str = "POOL_BALANCE";

#[contractimpl]
impl WaveCloudContract {
    pub fn initialize(env: Env, oracle_signer: Address) {
        env.storage().set(&Symbol::new(&env, ORACLE_SIGNER), &oracle_signer);
        env.storage().set(&Symbol::new(&env, POLICY_COUNT), &0i128);
        env.storage().set(&Symbol::new(&env, POOL_BALANCE), &0i128);
    }

    pub fn create_policy(env: Env, owner: Address, region: Symbol, payout_amount: i128, premium: i128, threshold_mm: i128) -> i128 {
        let count: i128 = env.storage().get_unchecked(&Symbol::new(&env, POLICY_COUNT)).unwrap_or(0i128);
        let id = count + 1;
        // store per-field to avoid complex serialization
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:owner", id)), &owner);
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:region", id)), &region);
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:payout", id)), &payout_amount);
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:premium", id)), &premium);
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:threshold", id)), &threshold_mm);
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:active", id)), &1i128);
        env.storage().set(&Symbol::new(&env, POLICY_COUNT), &id);
        id
    }

    pub fn get_policy_owner(env: Env, id: i128) -> Address {
        env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:owner", id))).unwrap()
    }

    pub fn deactivate_policy(env: Env, id: i128) {
        env.storage().set(&Symbol::new(&env, format!("POLICY:{}:active", id)), &0i128);
    }

    pub fn deposit_pool(env: Env, amount: i128) {
        let bal: i128 = env.storage().get_unchecked(&Symbol::new(&env, POOL_BALANCE)).unwrap_or(0i128);
        env.storage().set(&Symbol::new(&env, POOL_BALANCE), &(bal + amount));
        // note: actual token transfer should be implemented via token contract invocation
    }

    pub fn submit_oracle_reading(env: Env, oracle: Address, region: Symbol, rainfall_mm: i128, timestamp: i128, _signature: BytesN<32>) {
        // Verify oracle signer
        let signer: Address = env.storage().get_unchecked(&Symbol::new(&env, ORACLE_SIGNER)).unwrap();
        if oracle != signer { panic!("unauthorized oracle"); }
        // replay protection: simple timestamp check per region
        let last_ts_key = Symbol::new(&env, format!("LAST_TS:{}", region));
        let last_ts: i128 = env.storage().get_unchecked(&last_ts_key).unwrap_or(0i128);
        if timestamp <= last_ts { panic!("replay or stale reading"); }
        env.storage().set(&last_ts_key, &timestamp);

        // Evaluate policies in this region
        let count: i128 = env.storage().get_unchecked(&Symbol::new(&env, POLICY_COUNT)).unwrap_or(0i128);
        for i in 1..=count {
            let active: i128 = env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:active", i))).unwrap_or(0i128);
            if active == 0 { continue; }
            let policy_region: Symbol = env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:region", i))).unwrap();
            if policy_region != region { continue; }
            let threshold: i128 = env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:threshold", i))).unwrap_or(0i128);
            if rainfall_mm <= threshold {
                let payout: i128 = env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:payout", i))).unwrap_or(0i128);
                let pool: i128 = env.storage().get_unchecked(&Symbol::new(&env, POOL_BALANCE)).unwrap_or(0i128);
                if pool >= payout {
                    env.storage().set(&Symbol::new(&env, POOL_BALANCE), &(pool - payout));
                    let owner: Address = env.storage().get_unchecked(&Symbol::new(&env, format!("POLICY:{}:owner", i))).unwrap();
                    env.events().publish((Symbol::short("payout"),), (i, owner.clone(), payout));
                    env.storage().set(&Symbol::new(&env, format!("POLICY:{}:active", i)), &0i128);
                } else {
                    env.events().publish((Symbol::short("insolvent"),), (i, payout, pool));
                }
            }
        }
    }

    pub fn get_pool_balance(env: Env) -> i128 {
        env.storage().get_unchecked(&Symbol::new(&env, POOL_BALANCE)).unwrap_or(0i128)
    }
}

soroban_sdk::contractimport!("./spec.wasm");
