#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod owner;
pub mod private;
pub mod storage;
pub mod token;
pub mod types;
pub mod views;

use crate::types::*;

#[multiversx_sc::contract]
pub trait DynamicStakingAutoCompound:
    owner::OwnerModule
    + private::PrivateModule
    + storage::StorageModule
    + token::TokenModule
    + views::ViewsModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(
        &self,
        daily_reward: BigUint,
        hardcap_reward_per_share: BigUint,
        day0_start_time: u64,
        disabled: bool,
        opt_unstake_penalty: OptionalValue<u64>,
        opt_claim_time_penalty: OptionalValue<u64>,
        opt_accepted_token_id: OptionalValue<TokenIdentifier>,
    ) {
        match opt_unstake_penalty {
            OptionalValue::Some(val) => self.unlock_penalty_time().set(val),
            OptionalValue::None => {}
        }

        match opt_claim_time_penalty {
            OptionalValue::Some(val) => self.claim_penalty_time().set(val),
            OptionalValue::None => {}
        }

        match opt_accepted_token_id {
            OptionalValue::Some(val) => self.accepted_token_id().set(val),
            OptionalValue::None => {}
        }

        self.daily_reward().set_if_empty(&daily_reward);
        self.hardcap_reward_per_share()
            .set(hardcap_reward_per_share);
        self.reference_time().set_if_empty(day0_start_time);
        self.disabled().set_if_empty(disabled);
        self.current_staked_balance().set_if_empty(&BigUint::zero());
        self.compounded_balance().set_if_empty(&BigUint::zero());
        self.last_stake_balance_compound_day().set_if_empty(0u64);
        self.compute_rps();
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        // validate stake
        let tokens_sent = self.call_value().single_esdt();
        self.validate_stake_attempt(&tokens_sent);

        let staked_token_id = self.staked_token().get_token_id();
        let staked_token_nonce = self.create_staked_token(&tokens_sent.amount);
        self.send().direct(
            &self.blockchain().get_caller(),
            &EgldOrEsdtTokenIdentifier::esdt(staked_token_id),
            staked_token_nonce,
            &tokens_sent.amount,
        );

        if self.current_staked_balance().is_empty() {
            self.current_staked_balance().set(&tokens_sent.amount);
        } else {
            self.current_staked_balance()
                .update(|amount| *amount += &tokens_sent.amount);
        }

        // self.compute_rps(); endpoint for bot in owner module
        //to be called once per day

        if self.compounded_balance().is_empty() {
            self.compounded_balance().set(&tokens_sent.amount);
        } else {
            self.compounded_balance()
                .update(|amount| *amount += &tokens_sent.amount);
        }

        // self.compute_rps();
    }

    #[payable("*")]
    #[endpoint(unstake)]
    fn unstake(&self) {
        let tokens_sent = self.call_value().all_esdt_transfers();
        let mut payment_vec = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();

        //validate unstake, verify unstake penalty, etc.
        for token in tokens_sent.iter() {
            self.validate_transfer(&token);

            //calculate reward and send
            let stake_day = self.get_stake_day_from_attributes(token.token_nonce);
            let days_since_stake_started = self.get_days_since_start();
            self.require_can_unstake(stake_day, days_since_stake_started);

            let reward = self.compute_reward(&token.amount, stake_day);

            //burn stoken received
            self.burn_token(&token);

            //add to payment vec
            payment_vec.push(EsdtTokenPayment::new(
                self.accepted_token_id().get(),
                0u64,
                &reward + &token.amount,
            ));

            //update balance (already sent the amount of unstaked token)
            self.current_staked_balance()
                .update(|amount| *amount -= &token.amount);

            self.compounded_balance()
                .update(|amount| *amount -= &(token.amount + reward));
            // self.compute_rps();
        }

        //send all the payments through a multi payment
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &payment_vec);
    }

    #[payable("*")]
    #[endpoint(claim)]
    fn claim_reward(&self) {
        let tokens_sent = self.call_value().all_esdt_transfers();

        for token in tokens_sent.iter() {
            //validate claim
            self.validate_transfer(&token);

            //calculate reward
            let stake_day = self.get_stake_day_from_attributes(token.token_nonce);
            let days_since_stake_started = self.get_days_since_start();
            self.require_can_claim(stake_day, days_since_stake_started);
            let reward = self.compute_reward(&token.amount, stake_day);

            //send reward
            require!(&reward > &BigUint::zero(), "no reward to receive");

            //update balance
            self.compounded_balance()
                .update(|amount| *amount -= &reward);
            self.compute_rps();

            //burn staked token received
            self.burn_token(&token);

            //mint new staked token with today's nonce
            let new_staked_token_nonce = self.create_staked_token(&token.amount);
            let staked_token_id = self.staked_token().get_token_id();
            let accepted_token_id = self.accepted_token_id().get();

            // prepare payments
            let reward_payment = EsdtTokenPayment::new(accepted_token_id, 0, reward);

            let stake_payment =
                EsdtTokenPayment::new(staked_token_id, new_staked_token_nonce, token.amount);

            let mut payment_vec = ManagedVec::from_single_item(reward_payment);
            payment_vec.push(stake_payment);

            self.send()
                .direct_multi(&self.blockchain().get_caller(), &payment_vec);
        }
    }

    #[payable("*")]
    #[endpoint(merge)]
    fn merge_stakes(&self) {
        let tokens_sent = self.call_value().all_esdt_transfers();
        let mut total_reward = BigUint::zero();
        let mut total_new_staked_token = BigUint::zero();

        for token in tokens_sent.iter() {
            //validate
            self.validate_transfer(&token);

            //check reward
            let stake_day = self.get_stake_day_from_attributes(token.token_nonce);
            let reward = self.compute_reward(&token.amount, stake_day);

            //update balance
            self.compounded_balance()
                .update(|amount| *amount -= &reward);
            self.compute_rps();

            //add to to_send_reward_vec
            total_reward += reward;

            //add to the staked token amount that needs to be minted with today's nonce
            total_new_staked_token += &token.amount;

            //burn sent staked token
            self.burn_token(&token);
        }

        //prepare payment
        //total reward
        let total_reward_esdt_payment =
            EsdtTokenPayment::new(self.accepted_token_id().get(), 0u64, total_reward);

        let mut payment_vec = ManagedVec::from_single_item(total_reward_esdt_payment);

        //mint new staked token with today's nonce
        let new_staked_token_nonce = self.create_staked_token(&total_new_staked_token);
        let staked_token_id = self.staked_token().get_token_id();

        //new staked tokens tokens
        let new_staked_tokens = EsdtTokenPayment::new(
            staked_token_id,
            new_staked_token_nonce,
            total_new_staked_token,
        );

        payment_vec.push(new_staked_tokens);

        //send both tokens in one transaction
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &payment_vec);
    }
}
