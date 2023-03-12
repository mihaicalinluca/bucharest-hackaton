multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getDisabled)]
    #[storage_mapper("disabled")]
    fn disabled(&self) -> SingleValueMapper<bool>;

    #[view(getHardcapRewardPerShare)]
    #[storage_mapper("hardcapRewardPerShare")]
    fn hardcap_reward_per_share(&self) -> SingleValueMapper<BigUint>;

    #[view(getDailyReward)]
    #[storage_mapper("dailyReward")]
    fn daily_reward(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("rps")]
    fn reward_per_share(&self, day: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("tokenId")]
    fn accepted_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // #[storage_mapper("stakedTokenId")]
    // fn staked_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("stakedToken")]
    fn staked_token(&self) -> NonFungibleTokenMapper;

    #[storage_mapper("referenceTime")]
    fn reference_time(&self) -> SingleValueMapper<u64>;

    #[view(getUnlockPenaltyTime)]
    #[storage_mapper("unlockPenaltyTime")]
    fn unlock_penalty_time(&self) -> SingleValueMapper<u64>;

    #[view(getClaimPenaltyTime)]
    #[storage_mapper("claimPenaltyTime")]
    fn claim_penalty_time(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("currentStakedBalance")]
    fn current_staked_balance(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("real_rps")]
    fn real_rps(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("compounded_balance")]
    fn compounded_balance(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("last_stake_balance_compound_day")]
    fn last_stake_balance_compound_day(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("lastMintedDayToken")]
    fn last_minted_day_since_start(&self) -> SingleValueMapper<u64>;

    #[view(getLastStakedTokenNonce)]
    #[storage_mapper("lastSKosonNonce")]
    fn last_staked_token_nonce(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("stakedTokenName")]
    fn staked_token_name(&self) -> SingleValueMapper<ManagedBuffer>;
}
