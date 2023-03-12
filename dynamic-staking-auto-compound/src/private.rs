multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule {
    fn validate_stake_attempt(&self, tokens_sent: &EsdtTokenPayment<Self::Api>) {
        //REQUIRES
        require!(!self.disabled().get(), "maintenance");
        let accepted_token_id = self.accepted_token_id().get();
        require!(
            &tokens_sent.token_identifier == &accepted_token_id,
            "you can only stake TOKEN"
        );
        require!(&tokens_sent.amount > &BigUint::zero(), "no amount sent");
    }

    fn validate_transfer(&self, tokens_sent: &EsdtTokenPayment<Self::Api>) {
        require!(!self.disabled().get(), "maintenance");
        require!(
            &tokens_sent.token_identifier == &self.staked_token().get_token_id(),
            "you can only unstake using STAKEDTOKEN"
        );
        require!(&tokens_sent.amount > &BigUint::zero(), "no amount sent");
    }

    fn require_can_unstake(&self, stake_day: u64, current_days_since_start: u64) {
        require!(
            current_days_since_start - stake_day >= (self.unlock_penalty_time().get() / TEST_DAY), //replace with ONE_DAY
            "cannot unstake yet"
        );
    }

    fn require_can_claim(&self, stake_day: u64, current_days_since_start: u64) {
        require!(
            current_days_since_start - stake_day >= (self.claim_penalty_time().get() / TEST_DAY), //replace with ONE_DAY
            "cannot claim yet"
        );
    }

    //CALC FUNCTIONS
    fn compute_reward(&self, amount: &BigUint, stake_day: u64) -> BigUint {
        let days_since_start = self.get_days_since_start();
        if stake_day == days_since_start {
            return BigUint::zero();
        }

        let mut reward_per_share = BigUint::zero();
        let mut total_reward = (*amount).clone();

        let rps_denom = BigUint::from(RPS_DENOMINATION);
        for current_reward_day in stake_day..=days_since_start - 1 {
            if !self.reward_per_share(current_reward_day).is_empty() {
                reward_per_share = self.reward_per_share(current_reward_day).get();
            }
            // if &reward_per_share > &hardcap_reward_per_share {
            //     reward_per_share = hardcap_reward_per_share.clone();
            // }
            //reward per token * number of tokens owned by user / denomination (rps is already denominated)
            total_reward += &total_reward * &reward_per_share / &rps_denom;
        }

        &total_reward - amount
    }

    // fn get_and_compound_due_balance(&self, days_since_stake_started: u64, max_apr: &BigUint, daily_reward: &BigUint) -> BigUint {
    //     let rps_denom = BigUint::from(RPS_DENOMINATION);

    //     let last_compound_day;
    //     let last_compound_day_strg = self.last_stake_balance_compound_day();
    //     let mut compounded_balance;
    //     let compounded_balance_strg = self.compounded_balance();

    //     if !last_compound_day_strg.is_empty() {
    //         last_compound_day = self.last_stake_balance_compound_day().get();
    //     } else {
    //         last_compound_day = 0u64;
    //     }

    //     if days_since_stake_started == last_compound_day {
    //         return self.compounded_balance().get();
    //     }

    //     if compounded_balance_strg.is_empty() {
    //         compounded_balance = self.current_staked_balance().get();
    //     } else {
    //         compounded_balance = self.compounded_balance().get();
    //     }

    //     if &compounded_balance == &BigUint::zero() {
    //         return BigUint::zero();
    //     }

    //     // compound past days when compounding did not happen, and set reward_per_share accordingly
    //     for past_day_to_compound in last_compound_day..days_since_stake_started {
    //         let mut rps = daily_reward * &rps_denom / &compounded_balance;
    //         if &rps > max_apr {
    //             rps = max_apr.clone();
    //         }
    //         self.reward_per_share(past_day_to_compound).set(&rps);
    //         compounded_balance += &compounded_balance * &rps / &rps_denom;
    //     }

    //     self.compounded_balance().set(&compounded_balance);
    //     self.last_stake_balance_compound_day().set(&days_since_stake_started);

    //     return compounded_balance;
    // }

    fn get_and_compound_due_balance(
        &self,
        days_since_stake_started: u64,
        max_apr: &BigUint,
        daily_reward: &BigUint,
        current_day: u64,
    ) -> BigUint {
        let rps_denom = BigUint::from(RPS_DENOMINATION);

        let last_compound_day;
        let last_compound_day_strg = self.last_stake_balance_compound_day();
        let mut compounded_balance;
        let compounded_balance_strg = self.compounded_balance();

        if !last_compound_day_strg.is_empty() {
            last_compound_day = self.last_stake_balance_compound_day().get();
        } else {
            last_compound_day = 0u64;
        }

        if days_since_stake_started == last_compound_day {
            return self.compounded_balance().get();
        }

        if compounded_balance_strg.is_empty() {
            compounded_balance = self.current_staked_balance().get();
        } else {
            compounded_balance = self.compounded_balance().get();
        }

        if &compounded_balance == &BigUint::zero() {
            return BigUint::zero();
        }

        let mut rps = daily_reward * &rps_denom / &compounded_balance;
        if &rps > max_apr {
            rps = max_apr.clone();
        }
        self.reward_per_share(current_day).set(&rps);
        compounded_balance += &compounded_balance * &rps / &rps_denom;

        self.compounded_balance().set(&compounded_balance);
        self.last_stake_balance_compound_day()
            .set(&days_since_stake_started);

        return compounded_balance;
    }

    /////UTILS
    fn get_stake_day_from_attributes(&self, token_nonce: u64) -> u64 {
        let token_info = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &self.staked_token().get_token_id(),
            token_nonce,
        );

        let attributes: StakedTokenAttributes = token_info.decode_attributes();
        attributes.stake_day_since_start
    }

    #[view(getDaysSinceStart)]
    fn get_days_since_start(&self) -> u64 {
        (self.blockchain().get_block_timestamp() - self.reference_time().get()) / TEST_DAY
        //replace with ONE_DAY
    }
}
