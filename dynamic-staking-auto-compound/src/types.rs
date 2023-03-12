multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const NFT_AMOUNT: u32 = 1;
pub const ONE_DAY: u64 = 24 * 3600;
pub const ONE_WEEK: u64 = 7 * ONE_DAY; //604800;
pub const TEST_DAY: u64 = 60; //60 seconds
pub const KOSON_TOKEN_ID: &[u8] = b"KOSON-5dd4fa";
// pub const KOSON_TOKEN_ID: &[u8] = b"KOSON-5ac764";
pub const EGLD_NUM_DECIMALS: u32 = 18;
pub const RPS_DENOMINATION: u32 = 1_000_000_000;

//maybe
#[derive(
    ManagedVecItem, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, PartialEq,
)]
pub struct Token<M: ManagedTypeApi> {
    pub token: ManagedVec<M, EsdtTokenPayment<M>>,
    pub token_type: BigUint<M>,
}

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Debug,
)]
pub struct StakedTokenAttributes {
    pub stake_day_since_start: u64,
}
