multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule + crate::private::PrivateModule {
    //token issuance
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        // staked_token_name: ManagedBuffer,
    ) {
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        // self.staked_token_name().set_if_empty(&staked_token_name);

        self.issue_started_event(&caller, &token_ticker);
        // self.send()
        //     .esdt_system_sc_proxy()
        //     .register_meta_esdt(
        //         issue_cost,
        //         &token_display_name,
        //         &token_ticker,
        //         MetaTokenProperties {
        //             num_decimals: EGLD_NUM_DECIMALS as usize,
        //             can_freeze: true,
        //             can_wipe: true,
        //             can_pause: true,
        //             can_upgrade: true,
        //             can_add_special_roles: true,
        //             can_change_owner: false,
        //             can_transfer_create_role: true,
        //         },
        //     )
        //     .async_call()
        //     .with_callback(OwnerModule::callbacks(self).esdt_issue_callback(&caller))
        //     .call_and_exit()

        self.staked_token().issue_and_set_all_roles(
            EsdtTokenType::Meta,
            issue_cost,
            token_display_name,
            token_ticker,
            18usize,
            None,
        );
    }

    // #[callback]
    // fn esdt_issue_callback(
    //     &self,
    //     caller: &ManagedAddress,
    //     #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    // ) {
    //     match result {
    //         ManagedAsyncCallResult::Ok(token_identifier) => {
    //             self.issue_success_event(caller, &token_identifier);
    //             self.staked_token_id().set(&token_identifier);
    //         }
    //         ManagedAsyncCallResult::Err(message) => {
    //             let returned_token = self.call_value().egld_or_single_esdt();
    //             self.issue_failure_event(caller, &message.err_msg);

    //             // return issue cost to the owner
    //             // TODO: test that it works
    //             if returned_token.token_identifier.is_egld()
    //                 && returned_token.amount > BigUint::zero()
    //             {
    //                 self.send().direct_egld(caller, &returned_token.amount);
    //             }
    //         }
    //     }
    // }

    // #[only_owner]
    // #[endpoint(setLocalRoles)]
    // fn set_local_roles(&self) {
    //     require!(!self.staked_token_id().is_empty(), "Must issue token first");

    //     let roles = [
    //         EsdtLocalRole::Mint,
    //         EsdtLocalRole::NftCreate,
    //         EsdtLocalRole::NftAddQuantity,
    //         EsdtLocalRole::Burn,
    //         EsdtLocalRole::NftBurn,
    //         EsdtLocalRole::Transfer,
    //     ];

    //     self.send()
    //         .esdt_system_sc_proxy()
    //         .set_special_roles(
    //             &self.blockchain().get_sc_address(),
    //             &self.staked_token_id().get(),
    //             roles[..].iter().cloned(),
    //         )
    //         .async_call()
    //         .call_and_exit()
    // }

    // #[only_owner]
    // #[endpoint(setStakeTokenIdentifier)]
    // fn set_stake_token_identifier(
    //     &self,
    //     token: TokenIdentifier,
    //     last_minted_nonce: u64,
    //     last_minted_day: u64,
    // ) {
    //     self.staked_token_id().set(&token);
    //     self.last_staked_token_nonce().set(&last_minted_nonce);
    //     self.last_minted_day_since_start().set(&last_minted_day);
    // }
    #[only_owner]
    #[endpoint(setDailyReward)]
    fn set_daily_reward(&self, reward: BigUint) {
        self.daily_reward().set(reward);
    } //could also be done in init

    #[only_owner]
    #[endpoint(setRPSHardCap)]
    fn set_rps_hardcap(&self, hardcap_reward_per_share: BigUint) {
        self.hardcap_reward_per_share()
            .set(hardcap_reward_per_share);
    } //could also be done in init

    #[only_owner]
    #[endpoint(computeRps)]
    fn compute_rps(&self) {
        let days_since_stake_started = self.get_days_since_start();
        let max_apr = self.hardcap_reward_per_share();
        let daily_reward = self.daily_reward().get();
        let balance = self.get_and_compound_due_balance(
            days_since_stake_started,
            &max_apr.get(),
            &daily_reward,
            days_since_stake_started,
        );
        // this if should not make sense anymore
        if &balance == &BigUint::zero() {
            if max_apr.is_empty() {
                self.reward_per_share(days_since_stake_started)
                    .set(&BigUint::zero());
            } else {
                self.reward_per_share(days_since_stake_started)
                    .set(&max_apr.get());
            }
            return;
        }

        let mut reward_per_share = &daily_reward * &BigUint::from(RPS_DENOMINATION) / &balance;
        if !max_apr.is_empty() && &reward_per_share > &max_apr.get() {
            reward_per_share = max_apr.get();
        }
        self.reward_per_share(days_since_stake_started)
            .set(&reward_per_share);
    }

    #[only_owner]
    #[endpoint(enableStaking)]
    fn enable_contract(&self) {
        self.disabled().set(true); //&(disabled == 1u8)
    }

    #[only_owner]
    #[endpoint(disableStaking)]
    fn disable_contract(&self) {
        self.disabled().set(false); //&(disabled == 0u8)
    }

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);
}
