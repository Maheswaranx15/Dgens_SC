use anchor_lang::prelude::Pubkey;

pub const POOL_SEED: &str = "pool_";
pub const VAULT_SEED: &str = "vault_";
pub const OWNER_VAULT_SEED: &str = "ownervault_";
pub const USER_SEED: &str = "user_";
pub const CAMPAIGN_SEED: &str = "campaign_";

pub const OWNER_KEY: Pubkey = anchor_lang::solana_program::pubkey!("hFPVCyAan8HVuC3AVM34usCqUcEnqmDjdkHPk5NrAKW"); 

pub const MAX_REPORTER_COUNT: usize = 100;
pub const FIXED_SOL: u64 = 100000000;
pub const DECIMAL: u64 = 1000000000;
