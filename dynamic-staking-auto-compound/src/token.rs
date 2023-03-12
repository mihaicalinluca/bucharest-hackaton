multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait TokenModule: crate::storage::StorageModule + crate::private::PrivateModule {
    fn create_staked_token(&self, amount: &BigUint) -> u64 {
        let current_days_since_start = self.get_days_since_start();
        let last_minted_day = self.last_minted_day_since_start();

        // check if there was a nonce created for this day
        if last_minted_day.is_empty() || last_minted_day.get() != current_days_since_start {
            self.last_minted_day_since_start()
                .set(&current_days_since_start);
            return self.mint_token(amount, current_days_since_start);
        }

        return self.mint_token(amount, current_days_since_start);
    }

    fn mint_token(&self, amount: &BigUint, days_since_start: u64) -> u64 {
        let staked_token_id = self.staked_token().get_token_id();
        let attributes = StakedTokenAttributes {
            stake_day_since_start: days_since_start,
        };
        let minted_nonce =
            self.send()
                .esdt_nft_create_compact_named(&staked_token_id, amount, &self.staked_token_name().get(), &attributes);
        self.last_staked_token_nonce().set(&minted_nonce);
        self.last_minted_day_since_start().set(&days_since_start);

        minted_nonce
    }

    //needs ESDTLocalRoleBurn
    fn burn_token(&self, tokens_sent: &EsdtTokenPayment<Self::Api>) {
        self.send().esdt_local_burn(
            &tokens_sent.token_identifier,
            tokens_sent.token_nonce,
            &tokens_sent.amount,
        )
    }

    //needs ESDTLocalRoleNftAddQuantity
    fn add_quantity_to_token(&self, amount: &BigUint) -> u64 {
        let last_nonce = self.last_staked_token_nonce().get();
        self.send()
            .esdt_local_mint(&self.staked_token().get_token_id(), last_nonce, amount);

        last_nonce
    }

    
}