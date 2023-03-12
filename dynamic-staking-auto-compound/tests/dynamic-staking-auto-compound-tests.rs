multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use dynamic_staking_auto_compound::endpoints::stake;
use dynamic_staking_auto_compound::types::*;
use dynamic_staking_auto_compound::{
    owner::OwnerModule, private::PrivateModule, storage::StorageModule, views::ViewsModule,
    DynamicStakingAutoCompound,
};
use multiversx_sc_scenario::whitebox::*;
use multiversx_sc_scenario::*;

const WASM_PATH: &'static str = "../output/dynamic-staking-auto-compound.wasm";
const TOKEN_ID: &[u8] = b"TOKEN-5dd4fa";
const STAKED_TOKEN_ID: &[u8] = b"STAKEDTOKEN-123456";
const RPS: u64 = 1_000_000_000u64;
const DAILY_ISSUANCE: u64 = 10_000u64;

struct StakingSetup<StakingObjBuilder>
where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub staking_sc_wrapper:
        ContractObjWrapper<dynamic_staking_auto_compound::ContractObj<DebugApi>, StakingObjBuilder>,
    // pub owner_address: Address,
    pub client_address: Address,
    pub other_client_address: Address,
    pub other_sc_wrapper:
        ContractObjWrapper<dynamic_staking_auto_compound::ContractObj<DebugApi>, StakingObjBuilder>,
}

fn setup_staking<StakingObjBuilder>(
    cf_builder: StakingObjBuilder,
) -> StakingSetup<StakingObjBuilder>
where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let issuance_cost = rust_biguint!(500_000_000_000_000_000u64);
    let daily_reward: BigUint<DebugApi> = managed_biguint!(DAILY_ISSUANCE);
    let hardcap_reward_per_share: BigUint<DebugApi> = managed_biguint!(RPS); // 100.0000000%

    let mut blockchain_wrapper = BlockchainStateWrapper::new();

    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let other_client_address = blockchain_wrapper.create_user_account(&rust_zero);

    let client_address = blockchain_wrapper.create_user_account(&issuance_cost);
    let staking_sc_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    let other_sc_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    let user_token_balance = num_bigint::ToBigUint::to_biguint(&1_000_000).unwrap();
    let sc_token_balance = num_bigint::ToBigUint::to_biguint(&50_000_000).unwrap();
    blockchain_wrapper.set_esdt_balance(&client_address, TOKEN_ID, &user_token_balance);
    blockchain_wrapper.set_esdt_balance(
        &staking_sc_wrapper.address_ref(),
        TOKEN_ID,
        &sc_token_balance,
    );

    // deploy
    blockchain_wrapper
        .execute_tx(&owner_address, &staking_sc_wrapper, &rust_zero, |sc| {
            let day0_start_time = 0u64;
            let disabled = false;
            let opt_unstake_penalty = OptionalValue::Some(ONE_DAY); //one week
            let opt_claim_time_penalty = OptionalValue::Some(ONE_DAY); //one day

            sc.init(
                daily_reward.clone(),
                hardcap_reward_per_share.clone(),
                day0_start_time,
                disabled,
                opt_unstake_penalty,
                opt_claim_time_penalty,
                OptionalValue::Some(TOKEN_ID.into()),
            );
        })
        .assert_ok();

    // set roles for staked token
    let roles = [
        EsdtLocalRole::Mint,
        EsdtLocalRole::NftCreate,
        EsdtLocalRole::NftAddQuantity,
        EsdtLocalRole::Burn,
        EsdtLocalRole::NftBurn,
        EsdtLocalRole::Transfer,
    ];

    blockchain_wrapper.set_esdt_local_roles(
        staking_sc_wrapper.address_ref(),
        STAKED_TOKEN_ID,
        &roles[..],
    );

    // set daily rewards
    blockchain_wrapper
        .execute_tx(&owner_address, &staking_sc_wrapper, &rust_zero, |sc| {
            sc.set_daily_reward(daily_reward);
        })
        .assert_ok();

    // set rps hardcap (apr)
    blockchain_wrapper
        .execute_tx(&owner_address, &staking_sc_wrapper, &rust_zero, |sc| {
            sc.set_rps_hardcap(hardcap_reward_per_share);
        })
        .assert_ok();

    // blockchain_wrapper
    //     .execute_tx(&owner_address, &staking_sc_wrapper, &rust_zero, |sc| {
    //         sc.set_stake_token_identifier(managed_token_id!(STAKED_TOKEN_ID), 1, 0);
    //     })
    //     .assert_ok();

    StakingSetup {
        blockchain_wrapper,
        staking_sc_wrapper,
        // owner_address,
        client_address,
        other_client_address,
        other_sc_wrapper,
    }
}

#[test]
fn init_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = setup_staking(dynamic_staking_auto_compound::contract_obj);
    sc_setup
        .blockchain_wrapper
        .dump_state_for_account::<ManagedBuffer<DebugApi>>(&sc_setup.client_address);
    // check_compounded_balance(&mut sc_setup, 0);
    // check_uncompounded_balance(&mut sc_setup, 0);
    // check_rps(&mut sc_setup, 0, RPS);
}

#[test]
fn stake_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = setup_staking(dynamic_staking_auto_compound::contract_obj);
    let stake_day_number = 0;
    let mut day_number = stake_day_number;
    let stake_amount = 1_000;
    stake_and_check_balance(&mut sc_setup, stake_amount, day_number, 1); // 1000% APR, limited to rps = 100%

    //test transfer role
    // send_somewhere_other_than_sc(&mut sc_setup, stake_amount);
}

// #[test]
// fn unstake_test() {
//     let _ = DebugApi::dummy();
//     let mut sc_setup = setup_staking(dynamic_staking_auto_compound::contract_obj);
//     let stake_day_number = 0;
//     let mut day_number = stake_day_number;
//     let stake_amount = 1_000;

//     //day 0
//     stake_and_check_balance(&mut sc_setup, stake_amount, day_number, 1);
//     refresh_rps(&mut sc_setup);

//     //day 1
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 2
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 3
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 4
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 5
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 6
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     //day 7
//     day_number = add_day(&mut sc_setup, day_number);
//     refresh_rps(&mut sc_setup);

//     unstake_and_check(&mut sc_setup, 1u64, stake_amount, day_number, 44998);
// }

#[test]
fn test_complex_flow() {
    let _ = DebugApi::dummy();
    let mut sc_setup = setup_staking(dynamic_staking_auto_compound::contract_obj);
    let stake_day_number = 0;
    let mut day_number = stake_day_number;
    let stake_amount = 1_000;
    stake_and_check_balance(&mut sc_setup, stake_amount, day_number, 1); // 1000% APR, limited to rps = 100%
    refresh_rps(&mut sc_setup);

    day_number = add_day(&mut sc_setup, day_number); // day 1
    refresh_rps(&mut sc_setup); // enforce a compound

    day_number = add_day(&mut sc_setup, day_number); // day 2
    refresh_rps(&mut sc_setup); // enforce a compound

    day_number = add_day(&mut sc_setup, day_number); // day 3
    refresh_rps(&mut sc_setup); // enforce a compound

    day_number = add_day(&mut sc_setup, day_number); // day 4
    refresh_rps(&mut sc_setup); // enforce a compound

    day_number = add_day(&mut sc_setup, day_number); // day 5
    refresh_rps(&mut sc_setup); // enforce a compound

    let rewards = 25000;
    let skoson_nonce = 1;
    unstake_and_check(&mut sc_setup, skoson_nonce, 500, day_number, rewards / 2);
    refresh_rps(&mut sc_setup);

    check_compounded_balance(&mut sc_setup, 13000); // compounded balance must be equal to total staked amount, without rewards, since rewards were withdrawn already
    check_rps(&mut sc_setup, day_number, 769230769); // 10k / 13k = 0.769230769
    check_uncompounded_balance(&mut sc_setup, 500);

    unstake_and_check(&mut sc_setup, skoson_nonce, 250, day_number, rewards / 4);
    refresh_rps(&mut sc_setup);

    check_compounded_balance(&mut sc_setup, 6500); // 75% of initial staked removed
    check_rps(&mut sc_setup, day_number, RPS); // 10k / 6.5k > 1, so hardcap RPS is expected
    check_uncompounded_balance(&mut sc_setup, 250);

    day_number = add_day(&mut sc_setup, day_number); // day 5
    let stake_two_day_number = day_number;
    refresh_rps(&mut sc_setup); // enforce a compound
    stake_and_check_balance(&mut sc_setup, stake_amount, stake_two_day_number, 2); // 1000% APR, limited to rps = 100%
    refresh_rps(&mut sc_setup);

    check_uncompounded_balance(&mut sc_setup, 1250);
    check_compounded_balance(&mut sc_setup, 14000); // 6500 compounded + 100%, and 1000 extra stake: 13k + 1k = 14k
    check_rps(&mut sc_setup, day_number, 714285714); // 10k / 14k = 0.714285714
    check_rewards(&mut sc_setup, stake_day_number, 250, 12750); // 13k compounded by this batch - 250 staked amount
    check_rewards(&mut sc_setup, stake_two_day_number, 1000, 0);

    add_day(&mut sc_setup, day_number); // day 5
    check_rewards(&mut sc_setup, stake_day_number, 250, 22035); // 13k compounded by this batch * prev RPS of 71.428571% - 250 staked amount, rounded
    check_rewards(&mut sc_setup, stake_two_day_number, 1000, 714);
}

//UTILS
fn stake_and_check_balance<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    amount_u64: u64,
    day_number: u64,
    expected_nonce: u64,
) where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    let addr = &setup.client_address;
    let amount = num_bigint::ToBigUint::to_biguint(&amount_u64).unwrap();
    let initial_staked_token_balance =
        b_wrapper.get_esdt_balance(addr, STAKED_TOKEN_ID, expected_nonce);

    b_wrapper
        .execute_esdt_transfer(
            addr,
            &setup.staking_sc_wrapper,
            TOKEN_ID,
            0,
            &amount,
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();

    let new_staked_token_balance =
        b_wrapper.get_esdt_balance(addr, &STAKED_TOKEN_ID, expected_nonce);

    assert_eq!(
        initial_staked_token_balance + amount,
        new_staked_token_balance
    );

    let expected_attributes = StakedTokenAttributes {
        stake_day_since_start: day_number,
    };
    b_wrapper.check_nft_balance(
        addr,
        STAKED_TOKEN_ID,
        expected_nonce,
        &new_staked_token_balance,
        Some(&expected_attributes),
    );
}

fn unstake_and_check<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    staked_nonce: u64,
    amount_to_unstake: u64,
    _current_day: u64,
    expected_rewards: u64,
) where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    let addr = &setup.client_address;
    let amount = num_bigint::ToBigUint::to_biguint(&amount_to_unstake).unwrap();
    let initial_s_koson_balance = b_wrapper.get_esdt_balance(addr, STAKED_TOKEN_ID, staked_nonce);
    let initial_koson_balance = b_wrapper.get_esdt_balance(addr, TOKEN_ID, 0);

    let sc_balance =
        b_wrapper.get_esdt_balance(setup.staking_sc_wrapper.address_ref(), TOKEN_ID, 0);
    b_wrapper.set_esdt_balance(
        setup.staking_sc_wrapper.address_ref(),
        TOKEN_ID,
        &(&sc_balance + &num_bigint::ToBigUint::to_biguint(&expected_rewards).unwrap()),
    );

    b_wrapper
        .execute_esdt_transfer(
            addr,
            &setup.staking_sc_wrapper,
            STAKED_TOKEN_ID,
            staked_nonce,
            &amount,
            |sc| {
                sc.unstake();
            },
        )
        .assert_ok();

    let new_staked_token_balance = b_wrapper.get_esdt_balance(addr, STAKED_TOKEN_ID, staked_nonce);
    let new_token_balance = b_wrapper.get_esdt_balance(addr, TOKEN_ID, 0);
    let new_sc_koson_balance =
        b_wrapper.get_esdt_balance(setup.staking_sc_wrapper.address_ref(), TOKEN_ID, 0);
    let expected_received_rewards = num_bigint::ToBigUint::to_biguint(&expected_rewards).unwrap();

    assert_eq!(&initial_s_koson_balance - &amount, new_staked_token_balance);
    assert_eq!(
        &initial_koson_balance + &expected_received_rewards + &amount,
        new_token_balance
    );
    assert_eq!(&sc_balance - &amount, new_sc_koson_balance);

    let expected_attributes = StakedTokenAttributes {
        stake_day_since_start: staked_nonce - 1,
    };
    b_wrapper.check_nft_balance(
        addr,
        STAKED_TOKEN_ID,
        staked_nonce,
        &new_staked_token_balance,
        Some(&expected_attributes),
    );
}

fn send_somewhere_other_than_sc<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    amount_u64: u64,
) where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    let addr = &setup.client_address;
    let amount = num_bigint::ToBigUint::to_biguint(&amount_u64).unwrap();

    assert_ne!(
        setup.staking_sc_wrapper.address_ref(),
        setup.other_sc_wrapper.address_ref()
    );

    b_wrapper.dump_state_for_account::<ManagedBuffer<DebugApi>>(addr);

    b_wrapper
        .execute_esdt_transfer(
            addr,
            &setup.other_sc_wrapper,
            STAKED_TOKEN_ID,
            1,
            &amount,
            |sc| {},
        )
        .assert_ok(); //should be error, fix this

    b_wrapper.dump_state_for_account::<ManagedBuffer<DebugApi>>(addr);
}

fn add_day<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    crt_day_number: u64,
) -> u64
where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    b_wrapper.set_block_timestamp((crt_day_number + 1) * ONE_DAY + 1);
    crt_day_number + 1
}

fn refresh_rps<StakingObjBuilder>(setup: &mut StakingSetup<StakingObjBuilder>)
where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    b_wrapper
        .execute_tx(
            &setup.client_address,
            &setup.staking_sc_wrapper,
            &num_bigint::ToBigUint::to_biguint(&0u64).unwrap(),
            |sc| {
                sc.compute_rps();
            },
        )
        .assert_ok();
}

fn check_rps<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    day: u64,
    expected_rps: u64,
) where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    setup
        .blockchain_wrapper
        .execute_query(&setup.staking_sc_wrapper, |sc| {
            let rps = sc.get_reward_per_share(day);
            assert_eq!(managed_biguint!(expected_rps), rps);
        })
        .assert_ok();
}

fn check_compounded_balance<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    expected: u64,
) where
    StakingObjBuilder:
        'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    setup
        .blockchain_wrapper
        .execute_query(&setup.staking_sc_wrapper, |sc| {
            let compounded_balance = sc.get_compounded_balance();
            assert_eq!(managed_biguint!(expected), compounded_balance);
        })
        .assert_ok();
}

fn check_uncompounded_balance<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    expected: u64,
) where
    StakingObjBuilder: 'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    setup
        .blockchain_wrapper
        .execute_query(&setup.staking_sc_wrapper, |sc| {
            let balance = sc.get_uncompounded_balance();
            assert_eq!(managed_biguint!(expected), balance);
        })
        .assert_ok();
}

fn check_rewards<StakingObjBuilder>(
    setup: &mut StakingSetup<StakingObjBuilder>,
    stake_day_number: u64,
    stake_size: u64,
    expected_return: u64,
) where
    StakingObjBuilder: 'static + Copy + Fn() -> dynamic_staking_auto_compound::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;
    let staked_amount = managed_biguint!(stake_size);
    let expected_amount = managed_biguint!(expected_return);
    b_wrapper
        .execute_query(&setup.staking_sc_wrapper, |sc| {
            let pending_rewards = sc.get_pending_rewards(stake_day_number, staked_amount);
            assert_eq!(expected_amount, pending_rewards);
        })
        .assert_ok();
}

