use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::account::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct CreateOwnerVaultContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(init, seeds = [
    OWNER_VAULT_SEED.as_bytes(), 
    owner.key().as_ref()], 
    bump, 
    payer = owner, 
    space = size_of::<OwnerVault>() + 8
  )]
  pub owner_vault: AccountLoader<'info, OwnerVault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DepositContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
  pub owner_vault: AccountLoader<'info, OwnerVault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct WihdrawContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
  pub owner_vault: AccountLoader<'info, OwnerVault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct WihdrawAllContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
  pub owner_vault: AccountLoader<'info, OwnerVault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,
  #[account(init, seeds = [
    USER_SEED.as_bytes(), 
    owner.key().as_ref()], 
    bump, 
    payer = owner, 
    space = size_of::<User>() + 8
  )]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateAdminContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
    /// CHECK: it's not dangerous
  pub admin: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateSeniorContext<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
  #[account(mut)]
    /// CHECK: it's not dangerous
  pub senior: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditReporterContext<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub old_reporter: AccountInfo<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub reporter: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteReporterContext<'info> {
  #[account(mut)]
  pub admain: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub reporter: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateVaultContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(init, seeds = [
    VAULT_SEED.as_bytes(), 
    reporter.key().as_ref()], 
    bump, 
    payer = reporter, 
    space = size_of::<Vault>() + 8
  )]
  pub vault: AccountLoader<'info, Vault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditVaultContext<'info> {
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: Signer<'info>,
  #[account(mut)]
  pub vault: AccountLoader<'info, Vault>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(news_id: u64)]
pub struct CreateNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(init, seeds = [
    POOL_SEED.as_bytes(), 
    &news_id.to_le_bytes(), 
    reporter.key().as_ref()], 
    bump, 
    payer = reporter, 
    space = size_of::<Pool>() + 8
  )]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteNewsContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ApproveNewsContext<'info> {
  #[account(mut)]
  pub senior: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
}

#[derive(Accounts)]
pub struct DenyNewsContext<'info> {
  #[account(mut)]
  pub senior: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PublishNewsContext<'info> {
  #[account(mut,)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub pool: AccountLoader<'info, Pool>,
  #[account(mut)]
  pub vault: AccountLoader<'info, Vault>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayoutJuniorContext<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
    /// CHECK: it's not dangerous
  #[account(mut)]
  pub junior: AccountInfo<'info>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  #[account(mut)]
  pub vault: AccountLoader<'info, Vault>,
  #[account(mut)]
  pub owner_vault: AccountLoader<'info, OwnerVault>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendTipContext<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
    /// CHECK: it's not dangerous
  #[account(mut)]
  pub reporter: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMintFeeContext<'info> {
  #[account(mut)]
  pub reporter: Signer<'info>,
    /// CHECK: it's not dangerous
  #[account(mut, constraint = owner.key() == OWNER_KEY)]
  pub owner: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(campaign_id: u64)]
pub struct CreateCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub vault: AccountInfo<'info>,
  #[account(init, seeds = [
    CAMPAIGN_SEED.as_bytes(), 
    &campaign_id.to_le_bytes(), 
    advertiser.key().as_ref()], 
    bump, 
    payer = advertiser, 
    space = size_of::<CampaignPool>() + 8
  )]
  pub campaign_pool: AccountLoader<'info, CampaignPool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EditCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  #[account(mut)]
  pub campaign_pool: AccountLoader<'info, CampaignPool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteCampaignContext<'info> {
  #[account(mut)]
  pub advertiser: Signer<'info>,
  #[account(mut)]
  pub campaign_pool: AccountLoader<'info, CampaignPool>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ApproveCampaignContext<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
  #[account(mut)]
  pub vault: Signer<'info>,
  #[account(mut)]
  pub campaign_pool: AccountLoader<'info, CampaignPool>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DenyCampaignContext<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
  /// CHECK: it's not dangerous
  #[account(mut)]
  pub advertiser: AccountInfo<'info>,
  #[account(mut)]
  pub vault: Signer<'info>,
  #[account(mut)]
  pub campaign_pool: AccountLoader<'info, CampaignPool>,
  #[account(mut)]
  pub user: AccountLoader<'info, User>,
  pub system_program: Program<'info, System>
}