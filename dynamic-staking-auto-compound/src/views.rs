multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule: crate::storage::StorageModule + crate::private::PrivateModule {
    #[view(getReferenceTime)]
    fn get_reference_time(&self) -> u64 {
        self.reference_time().get()
    }

    #[view(getSKosonTokenId)]
    fn get_staked_token_id(&self) -> TokenIdentifier {
        self.staked_token().get_token_id()
    }

    // #[view(hasRoles)]
    // fn has_roles_set(&self) -> bool {
    //     let roles = self.blockchain().get_esdt_local_roles(&self.staked_token_id().get());
    //     roles.has_role(&EsdtLocalRole::Mint) &&
    //     roles.has_role(&EsdtLocalRole::NftAddQuantity) &&
    //     roles.has_role(&EsdtLocalRole::Burn)
    // }

    #[view(getPendingRewards)]
    fn get_pending_rewards(&self, stake_day: u64, amount: BigUint) -> BigUint {
        // let stake_day = self.get_stake_day_from_attributes(nonce);
        self.compute_reward(&amount, stake_day)
    }

    #[view(getLastRPS)]
    fn get_last_rps(&self) -> BigUint {
        let mut day = self.get_days_since_start();
        while self.reward_per_share(day).is_empty() {
            if day == 0 {
                return self.hardcap_reward_per_share().get();
            }
            day -= 1;
        }
        return self.reward_per_share(day).get();
    }

    #[view(getRewardPerShare)]
    fn get_reward_per_share(&self, day: u64) -> BigUint {
        return self.reward_per_share(day).get();
    }

    #[view(getCompoundedBalance)]
    fn get_compounded_balance(&self) -> BigUint {
        return self.compounded_balance().get();
    }

    #[view(getUncompoundedBalance)]
    fn get_uncompounded_balance(&self) -> BigUint {
        return self.current_staked_balance().get();
    }

    // #[view(getApproximateApy)]
    // fn get_approximate_apy(&self) -> BigUint {
    //     let last_rps = self.get_last_rps();
    //     return self.get_apr_from_apy(&last_rps);
    // }
}
