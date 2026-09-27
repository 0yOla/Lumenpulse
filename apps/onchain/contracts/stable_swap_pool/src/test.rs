use super::*;
use errors::StableSwapError;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Env, Symbol};

fn setup_pool<'a>(
    env: &Env,
) -> (
    Address,            // admin
    Address,            // user1
    Address,            // user2
    Address,            // token_a
    Address,            // token_b
    StellarAssetClient, // token_a_client
    StellarAssetClient, // token_b_client
    StableSwapPoolContractClient,
) {
    let admin = Address::generate(env);
    let user1 = Address::generate(env);
    let user2 = Address::generate(env);

    let token_admin = Address::generate(env);
    let token_a_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_b_id = env.register_stellar_asset_contract_v2(token_admin);

    let token_a_client = StellarAssetClient::new(env, &token_a_id);
    let token_b_client = StellarAssetClient::new(env, &token_b_id);

    // Mint tokens to users
    token_a_client.mint(&user1, &1_000_000_000_000);
    token_a_client.mint(&user2, &1_000_000_000_000);
    token_b_client.mint(&user1, &1_000_000_000_000);
    token_b_client.mint(&user2, &1_000_000_000_000);

    let pool_id = env.register(StableSwapPoolContract, ());
    let client = StableSwapPoolContractClient::new(env, &pool_id);

    client.initialize(&admin, &token_a_id.address(), &token_b_id.address());

    (
        admin,
        user1,
        user2,
        token_a_id.address(),
        token_b_id.address(),
        token_a_client,
        token_b_client,
        client,
    )
}

#[test]
fn test_initialize_pool() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_a_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_b_id = env.register_stellar_asset_contract_v2(token_admin);

    let pool_id = env.register(StableSwapPoolContract, ());
    let client = StableSwapPoolContractClient::new(&env, &pool_id);

    client.initialize(&admin, &token_a_id.address(), &token_b_id.address());

    // Verify storage
    let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    assert_eq!(stored_admin, admin);

    let stored_token_a: Address = env.storage().instance().get(&DataKey::TokenA).unwrap();
    assert_eq!(stored_token_a, token_a_id.address());

    let stored_token_b: Address = env.storage().instance().get(&DataKey::TokenB).unwrap();
    assert_eq!(stored_token_b, token_b_id.address());
}

#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_a_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_b_id = env.register_stellar_asset_contract_v2(token_admin);

    let pool_id = env.register(StableSwapPoolContract, ());
    let client = StableSwapPoolContractClient::new(&env, &pool_id);

    client.initialize(&admin, &token_a_id.address(), &token_b_id.address());
    let result = client.try_initialize(&admin, &token_a_id.address(), &token_b_id.address());
    assert_eq!(result, Err(Ok(StableSwapError::AlreadyInitialized)));
}

#[test]
fn test_add_liquidity_first_deposit() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add initial liquidity
    let lp_tokens = client.add_liquidity(&100_000_000, &100_000_000, &0);

    // Should mint LP tokens equal to geometric mean
    // sqrt(100M * 100M) = 100M
    assert_eq!(lp_tokens, 100_000_000);

    // Check reserves
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 100_000_000);
    assert_eq!(reserve_b, 100_000_000);

    // Check user LP balance
    assert_eq!(client.lp_balance(&user1), 100_000_000);

    // Check token balances
    assert_eq!(token_a_client.balance(&user1), 900_000_000_000);
    assert_eq!(token_b_client.balance(&user1), 900_000_000_000);
}

#[test]
fn test_add_liquidity_second_deposit() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, token_a_client, token_b_client, client) = setup_pool(&env);

    // First deposit
    client.add_liquidity(&100_000_000, &100_000_000, &0);

    // Second deposit - proportional
    let lp_tokens = client.add_liquidity(&50_000_000, &50_000_000, &0);

    // Should mint LP proportional to share: (50M * 100M) / (100M + 1) ≈ 50M
    assert!(lp_tokens > 49_000_000 && lp_tokens < 51_000_000);

    // Check reserves
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 150_000_000);
    assert_eq!(reserve_b, 150_000_000);

    // Check user LP balance
    assert_eq!(client.lp_balance(&user1), 100_000_000 + lp_tokens);
}

#[test]
fn test_add_liquidity_zero_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    let result = client.try_add_liquidity(&0, &100_000_000, &0);
    assert_eq!(result, Err(Ok(StableSwapError::InvalidAmount)));
}

#[test]
fn test_add_liquidity_slippage_protection() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    // Set min_lp higher than what would be minted
    let result = client.try_add_liquidity(&100_000_000, &100_000_000, &200_000_000);
    assert_eq!(result, Err(Ok(StableSwapError::SlippageExceeded)));
}

#[test]
fn test_remove_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&100_000_000, &100_000_000, &0);

    // Remove half
    let (out_a, out_b) = client.remove_liquidity(&50_000_000, &0, &0);

    // Should get half of each reserve
    assert_eq!(out_a, 50_000_000);
    assert_eq!(out_b, 50_000_000);

    // Check reserves
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 50_000_000);
    assert_eq!(reserve_b, 50_000_000);

    // Check user LP balance
    assert_eq!(client.lp_balance(&user1), 50_000_000);

    // Check token balances returned
    assert_eq!(token_a_client.balance(&user1), 950_000_000_000);
    assert_eq!(token_b_client.balance(&user1), 950_000_000_000);
}

#[test]
fn test_remove_liquidity_insufficient_lp() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&100_000_000, &100_000_000, &0);
    // Try to remove more LP than owned
    let result = client.try_remove_liquidity(&150_000_000, &0, &0);
    assert_eq!(result, Err(Ok(StableSwapError::InsufficientBalance)));
}

#[test]
fn test_remove_liquidity_slippage_protection() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&100_000_000, &100_000_000, &0);
    // Set min amounts higher than what would be received
    let result = client.try_remove_liquidity(&50_000_000, &60_000_000, &60_000_000);
    assert_eq!(result, Err(Ok(StableSwapError::SlippageExceeded)));
}

#[test]
fn test_swap_a_to_b() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Swap A for B
    let amount_in = 10_000_000; // 10 tokens
    let amount_out = client.swap(&token_a, &amount_in, &0);

    // With 0.04% fee and equal reserves: amount_out = (1B * 10M * 0.9996) / (1B + 10M * 0.9996)
    // ≈ 9,986,013
    assert!(amount_out > 9_900_000 && amount_out < 10_000_000);

    // Check reserves updated
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 1_010_000_000);
    assert!(reserve_b < 1_000_000_000);

    // Check user balances
    assert_eq!(token_a_client.balance(&user1), 990_000_000_000);
    assert_eq!(
        token_b_client.balance(&user1),
        1_000_000_000_000 + amount_out
    );
}

#[test]
fn test_swap_b_to_a() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Swap B for A
    let amount_in = 10_000_000;
    let amount_out = client.swap(&token_b, &amount_in, &0);

    assert!(amount_out > 9_900_000 && amount_out < 10_000_000);

    // Check reserves
    let (reserve_a, reserve_b) = client.get_reserves();
    assert!(reserve_a < 1_000_000_000);
    assert_eq!(reserve_b, 1_010_000_000);
}

#[test]
fn test_swap_zero_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);
    let result = client.try_swap(&token_a, &0, &0);
    assert_eq!(result, Err(Ok(StableSwapError::InvalidAmount)));
}

#[test]
fn test_swap_slippage_protection() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);
    // Set min_out higher than possible output
    let result = client.try_swap(&token_a, &10_000_000, &10_000_000);
    assert_eq!(result, Err(Ok(StableSwapError::SlippageExceeded)));
}

#[test]
fn test_pool_invariant_holds_after_swaps() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, _, _, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Initial reserves product (constant product invariant check)
    let (init_a, init_b) = client.get_reserves();
    let initial_k = init_a * init_b;

    // Perform multiple swaps
    for _ in 0..10 {
        client.swap(&token_a, &1_000_000, &0);
    }

    let (final_a, final_b) = client.get_reserves();
    let final_k = final_a * final_b;

    // K should increase due to fees (invariant holds with fees)
    assert!(final_k >= initial_k);
}

#[test]
fn test_fee_accounting_conserved() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Track total value in pool
    let (init_a, init_b) = client.get_reserves();
    let initial_total = init_a + init_b;

    // Perform swap
    let amount_in = 10_000_000;
    client.swap(&token_a, &amount_in, &0);

    let (final_a, final_b) = client.get_reserves();
    let final_total = final_a + final_b;

    // Total should increase by fee amount (0.04% of 10M = 4000)
    // But due to integer division, it's approximate
    assert!(final_total > initial_total);
    assert!(final_total - initial_total >= 3_900); // ~0.04% fee
}

#[test]
fn test_extreme_imbalance() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add highly imbalanced liquidity
    client.add_liquidity(&1_000_000_000, &100_000_000, &0);

    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 1_000_000_000);
    assert_eq!(reserve_b, 100_000_000);

    // Swap small amount from A to B (high price impact)
    let amount_in = 1_000_000;
    let amount_out = client.swap(&token_a, &amount_in, &0);

    // Should still work and not panic
    assert!(amount_out > 0);
    assert!(amount_out < amount_in); // Price impact

    // Check reserves updated
    let (new_a, new_b) = client.get_reserves();
    assert_eq!(new_a, 1_001_000_000);
    assert_eq!(new_b, 100_000_000 - amount_out);
}

#[test]
fn test_near_empty_pool_swap() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add minimal liquidity
    client.add_liquidity(&10_000_000, &10_000_000, &0);

    // Try to swap more than reserve
    let amount_in = 20_000_000;
    let result = client.try_swap(&token_a, &amount_in, &0);

    // Should fail with slippage or handle gracefully
    // With our formula: amount_out = (10M * 20M * 0.9996) / (10M + 20M * 0.9996) ≈ 6.66M
    // So it should succeed if min_out is 0
    assert!(result.is_ok());
}

#[test]
fn test_multiple_users_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, user2, _, _, token_a_client, token_b_client, client) = setup_pool(&env);

    // User1 adds liquidity
    env.mock_auths(&[user1.clone()]);
    client.add_liquidity(&100_000_000, &100_000_000, &0);

    // User2 adds liquidity
    env.mock_auths(&[user2.clone()]);
    let lp_user2 = client.add_liquidity(&50_000_000, &50_000_000, &0);

    // Check individual balances
    assert_eq!(client.lp_balance(&user1), 100_000_000);
    assert_eq!(client.lp_balance(&user2), lp_user2);

    // Total supply
    let total_supply: i128 = env.storage().persistent().get(&DataKey::LPSupply).unwrap();
    assert_eq!(total_supply, 100_000_000 + lp_user2);

    // Both remove liquidity
    env.mock_auths(&[user1.clone()]);
    client.remove_liquidity(&50_000_000, &0, &0);

    env.mock_auths(&[user2.clone()]);
    client.remove_liquidity(&lp_user2, &0, &0);

    // Pool should be empty
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 0);
    assert_eq!(reserve_b, 0);
}

#[test]
fn test_round_trip_add_remove_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, token_a_client, token_b_client, client) = setup_pool(&env);

    // Add liquidity
    client.add_liquidity(&100_000_000, &100_000_000, &0);

    // Remove all liquidity
    let lp_balance = client.lp_balance(&user1);
    let (out_a, out_b) = client.remove_liquidity(&lp_balance, &0, &0);

    // Should get back original amounts (minus any rounding)
    assert_eq!(out_a, 100_000_000);
    assert_eq!(out_b, 100_000_000);

    // Check token balances restored
    assert_eq!(token_a_client.balance(&user1), 1_000_000_000_000);
    assert_eq!(token_b_client.balance(&user1), 1_000_000_000_000);

    // Pool should be empty
    let (reserve_a, reserve_b) = client.get_reserves();
    assert_eq!(reserve_a, 0);
    assert_eq!(reserve_b, 0);
}

#[test]
fn test_sequential_swaps_maintain_ordering() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Swap A -> B
    let out1 = client.swap(&token_a, &10_000_000, &0);
    let (reserve_a1, reserve_b1) = client.get_reserves();

    // Swap B -> A
    let out2 = client.swap(&token_b, &10_000_000, &0);
    let (reserve_a2, reserve_b2) = client.get_reserves();

    // After round trip, reserves should be slightly higher due to fees
    assert!(reserve_a2 > reserve_a1);
    assert!(reserve_b2 > reserve_b1);
}

#[test]
fn test_isqrt_accuracy() {
    let env = Env::default();

    // Test perfect squares
    assert_eq!(StableSwapPoolContract::isqrt(0), 0);
    assert_eq!(StableSwapPoolContract::isqrt(1), 1);
    assert_eq!(StableSwapPoolContract::isqrt(4), 2);
    assert_eq!(StableSwapPoolContract::isqrt(100), 10);
    assert_eq!(StableSwapPoolContract::isqrt(10000), 100);
    assert_eq!(StableSwapPoolContract::isqrt(1_000_000), 1000);

    // Test non-perfect squares (floor of sqrt)
    assert_eq!(StableSwapPoolContract::isqrt(2), 1);
    assert_eq!(StableSwapPoolContract::isqrt(3), 1);
    assert_eq!(StableSwapPoolContract::isqrt(15), 3);
    assert_eq!(StableSwapPoolContract::isqrt(99), 9);

    // Large numbers
    assert_eq!(
        StableSwapPoolContract::isqrt(u128::MAX),
        340282366920938463463374607431768211455
    );
}

#[test]
fn test_reentrancy_guard_on_add_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    // Set reentrancy lock manually
    env.as_contract(&client.address, || {
        env.storage()
            .instance()
            .set(&symbol_short!("REENTRANT"), &true);
    });

    let result = client.try_add_liquidity(&100_000_000, &100_000_000, &0);
    assert_eq!(result, Err(Ok(StableSwapError::Reentrancy)));
}

#[test]
fn test_reentrancy_guard_on_swap() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Set reentrancy lock manually
    env.as_contract(&client.address, || {
        env.storage()
            .instance()
            .set(&symbol_short!("REENTRANT"), &true);
    });

    let result = client.try_swap(&token_a, &10_000_000, &0);
    assert_eq!(result, Err(Ok(StableSwapError::Reentrancy)));
}

#[test]
fn test_ttl_extension_on_operations() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, _, _, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Advance ledger past threshold
    env.ledger().set_sequence_number(LEDGER_THRESHOLD + 1);

    // Read operations should extend TTL
    let (ra, rb) = client.get_reserves();
    assert_eq!(ra, 1_000_000_000);
    assert_eq!(rb, 1_000_000_000);

    let lp_bal = client.lp_balance(&user1);
    assert_eq!(lp_bal, 1_000_000_000);

    // Advance further - should still work if TTL was extended
    env.ledger().set_sequence_number(2 * LEDGER_THRESHOLD + 2);

    let (ra2, rb2) = client.get_reserves();
    assert_eq!(ra2, 1_000_000_000);
    assert_eq!(rb2, 1_000_000_000);
}

#[test]
fn test_swap_fee_calculation() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Swap with known fee rate (0.04% = 4bp)
    let amount_in = 100_000_000; // 1000 tokens
    let amount_out = client.swap(&token_a, &amount_in, &0);

    // Without fee: (1B * 100M) / (1B + 100M) = 90,909,090
    // With 0.04% fee: amount_after_fee = 100M * 0.9996 = 99,960,000
    // amount_out = (1B * 99,960,000) / (1B + 99,960,000) ≈ 90,872,735
    let expected_no_fee = (1_000_000_000 * 100_000_000) / (1_000_000_000 + 100_000_000);
    let expected_with_fee = (1_000_000_000 * 99_960_000) / (1_000_000_000 + 99_960_000);

    assert!(amount_out < expected_no_fee);
    assert!(amount_out >= expected_with_fee - 1000); // Allow small rounding
    assert!(amount_out <= expected_with_fee);
}

#[test]
fn test_minimum_output_enforcement() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, user1, _, token_a, token_b, _, _, client) = setup_pool(&env);

    client.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Set min_out to exactly what we'd get - should succeed
    let amount_in = 10_000_000;
    let expected_out = client.swap(&token_a, &amount_in, &0);

    // Reset pool
    let pool_id = env.register(StableSwapPoolContract, ());
    let client2 = StableSwapPoolContractClient::new(&env, &pool_id);
    let admin = Address::generate(&env);
    client2.initialize(&admin, &token_a, &token_b);
    client2.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Should succeed with exact expected output
    let result = client2.try_swap(&token_a, &amount_in, &expected_out);
    assert!(result.is_ok());

    // Reset pool again
    let pool_id3 = env.register(StableSwapPoolContract, ());
    let client3 = StableSwapPoolContractClient::new(&env, &pool_id3);
    let admin3 = Address::generate(&env);
    client3.initialize(&admin3, &token_a, &token_b);
    client3.add_liquidity(&1_000_000_000, &1_000_000_000, &0);

    // Should fail with min_out = expected + 1
    let result = client3.try_swap(&token_a, &amount_in, &(expected_out + 1));
    assert!(result.is_err());
}

// Import DataKey for tests
use storage::{DataKey, LEDGER_BUMP, LEDGER_THRESHOLD};
