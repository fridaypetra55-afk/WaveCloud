#![cfg(test)]

use soroban_sdk::{Env, Address, Symbol};
use crate::WaveCloudContractClient;

#[test]
fn test_policy_lifecycle() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::WaveCloudContract {});
    let client = WaveCloudContractClient::new(&env, &contract_id);

    let oracle = Address::random(&env);
    client.initialize(&oracle);

    let owner = Address::random(&env);
    let region = Symbol::new(&env, "R_0_0");
    let id = client.create_policy(&owner, &region, &100i128, &10i128, &50i128);
    assert!(id >= 1);

    client.deposit_pool(&1000i128);
    // submit oracle reading that triggers payout (rainfall <= threshold)
    client.submit_oracle_reading(&oracle, &region, &10i128, &1i128, &soroban_sdk::BytesN::from_array(&env, &[0u8;32]));
    let pool = client.get_pool_balance();
    assert_eq!(pool, 900i128);
}
