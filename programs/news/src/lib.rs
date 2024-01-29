use anchor_lang::prelude::*;

pub mod contexts;
pub mod utils;
pub mod constants;
pub mod account;
pub mod errors;

use contexts::*;
use utils::*;
use errors::*;
use constants::*;

declare_id!("8LDu2mVLHTwomnBwXv5juTwnhDtjKndE6Z1U8BYSnzJh");

#[program]
pub mod news {
    use super::*;

    use anchor_lang::AccountsClose;


    pub fn create_owner_vault(
        ctx: Context<CreateOwnerVaultContext>
    ) -> Result<()> {
        let mut a_owner_vault = ctx.accounts.owner_vault.load_init()?;
        let a_owner = &ctx.accounts.owner;

        a_owner_vault.owner = a_owner.to_account_info().key();
        a_owner_vault.balance = 0;

        Ok(())
    }

    pub fn deposit(
        ctx: Context<DepositContext>,
        amount: u64
    ) -> Result<()> {
        {
            let owner_vault = &ctx.accounts.owner_vault;

            let system_program = &ctx.accounts.system_program;
    
            require!(amount > 0,  NewsError::InvalidPrice);
            
            let cpi_ctx = CpiContext::new(
                system_program.to_account_info().clone(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info().clone(),
                    to: owner_vault.to_account_info().clone()
                }
            );
    
            anchor_lang::system_program::transfer(cpi_ctx, amount)?;
            drop(owner_vault);
        }
        {
            let mut a_owner_vault = ctx.accounts.owner_vault.load_mut()?;
            a_owner_vault.balance += amount;
        }

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<WihdrawContext>,
        amount: u64
    ) -> Result<()> {
        {
            let owner_vault = ctx.accounts.owner_vault.load()?;
            let balance = owner_vault.balance;
            require!(balance > 0,  NewsError::InsufficientFunds);
        }
        {
            let _owner_vault = &mut ctx.accounts.owner_vault;
            let owner = &mut ctx.accounts.owner;

            let from = _owner_vault.to_account_info();
            let to = owner.to_account_info();
            **from.try_borrow_mut_lamports()? -= amount;
            **to.try_borrow_mut_lamports()? += amount;
        }
        {
            let mut a_owner_vault = ctx.accounts.owner_vault.load_mut()?;
            a_owner_vault.balance = a_owner_vault.balance - amount;
        }
    } 
        Ok(())   
    }

    pub fn withdraw_all(
        ctx: Context<WihdrawAllContext>
    ) -> Result<()> {
        {
            {
                let owner_vault = ctx.accounts.owner_vault.load()?;
                let balance = owner_vault.balance;
                require!(balance > 0,  NewsError::InsufficientFunds);
            }
            {
                let _owner_vault = &mut ctx.accounts.owner_vault;
                let owner = &mut ctx.accounts.owner;

                let from = _owner_vault.to_account_info();
                let to = owner.to_account_info();
                **from.try_borrow_mut_lamports()? -= ctx.accounts.owner_vault.load()?.balance;
                **to.try_borrow_mut_lamports()? += ctx.accounts.owner_vault.load()?.balance;
            }
            {
                let mut a_owner_vault = ctx.accounts.owner_vault.load_mut()?;
                a_owner_vault.balance = 0;
            }
        } 
        Ok(())
    }

    pub fn create_user(
        ctx: Context<CreateUserContext>
    ) -> Result<()> {
        let mut a_user = ctx.accounts.user.load_init()?;
        a_user.count = 0;

        Ok(())
    }

    pub fn create_admin(
        ctx: Context<CreateAdminContext>
    ) -> Result<()> {
        let mut a_user = ctx.accounts.user.load_mut()?;
        let a_admin = &ctx.accounts.admin;

        require!(
            (a_user.count as usize) < MAX_REPORTER_COUNT, 
            NewsError::OverMaxCount
        );

        let result: bool = a_user.add_reporter(a_admin.to_account_info().key(), 2)?;
        require!(result, NewsError::CreateReporterError);

        a_user.count += 1;

        Ok(())
    }

    pub fn create_senior(
        ctx: Context<CreateSeniorContext>
    ) -> Result<()> {
        let mut a_user = ctx.accounts.user.load_mut()?;
        let a_admin = &ctx.accounts.admin;
        let a_senior = &ctx.accounts.senior;

        require!(
            (a_user.count as usize) < MAX_REPORTER_COUNT, 
            NewsError::OverMaxCount
        );

        let is_admin: bool = a_user.validate_reporter(a_admin.to_account_info().key(), 2)?;
        require!( is_admin,  NewsError::NotAdmin );

        let result: bool = a_user.add_reporter(a_senior.to_account_info().key(), 1)?;
        require!(result, NewsError::CreateReporterError);

        a_user.count += 1;

        Ok(())
    }

    pub fn edit_reporter(
        ctx: Context<EditReporterContext>,
        role: u32
    ) -> Result<()> {
        let mut a_user = ctx.accounts.user.load_mut()?;
        let a_old_reporter = &ctx.accounts.old_reporter;
        let a_reporter = &ctx.accounts.reporter;

        let result: bool = a_user.edit_reporter(a_old_reporter.to_account_info().key(), a_reporter.to_account_info().key(), role)?;
        require!(result, NewsError::EditReporterError);

        Ok(())
    }

    pub fn delete_reporter(
        ctx: Context<DeleteReporterContext>
    ) -> Result<()> {
        let mut a_user = ctx.accounts.user.load_mut()?;
        let a_reporter = &ctx.accounts.reporter;

        let result: bool = a_user.delete_reporter(a_reporter.to_account_info().key())?;
        require!(result, NewsError::DeleteReporterError);
        a_user.count -= 1;

        Ok(())
    }

    pub fn create_vault(
        ctx: Context<CreateVaultContext>
    ) -> Result<()> {
        let mut a_vault = ctx.accounts.vault.load_init()?;
        let a_reporter = &ctx.accounts.reporter;

        a_vault.reporter = a_reporter.to_account_info().key();
        a_vault.balance = 0;
        Ok(())
    }

    pub fn edit_vault(
        ctx: Context<EditVaultContext>,
        price: u64
    ) -> Result<()> {
        let mut a_vault = ctx.accounts.vault.load_mut()?;

        a_vault.balance = price;
        Ok(())
    }

    pub fn create_news(
        ctx: Context<CreateNewsContext>, 
        news_id: u64,
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_init()?;
        let a_reporter = &ctx.accounts.reporter;
        let current_time = get_current_time()?;

        a_pool.news_id = news_id;
        a_pool.reporter = a_reporter.to_account_info().key();
        a_pool.created_at = current_time;
        a_pool.updated_at = current_time;
        a_pool.state = 0;
        Ok(())
    }

    pub fn edit_news(
        ctx: Context<EditNewsContext>, 
        news_id: u64, 
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let current_time = get_current_time()?;

        a_pool.news_id = news_id;
        a_pool.updated_at = current_time;
        a_pool.state = 1;
        Ok(())
    }

    pub fn delete_news(
        ctx: Context<DeleteNewsContext>
    ) -> Result<()> {
        let a_reporter = &ctx.accounts.reporter;

        ctx.accounts.pool.close(a_reporter.to_account_info())?;
        
        Ok(())
    }

    pub fn approve_news(
        ctx: Context<ApproveNewsContext>
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;

        let a_user = ctx.accounts.user.load_mut()?;
        let a_senior = &ctx.accounts.senior;

        let is_senior: bool = a_user.validate_reporter(a_senior.to_account_info().key(), 1)?;
        require!( is_senior,  NewsError::NotSeniorReporter);

        a_pool.state = 2;
        Ok(())
    }

    pub fn deny_news(
        ctx: Context<DenyNewsContext>
    ) -> Result<()> {
        let mut a_pool = ctx.accounts.pool.load_mut()?;

        let a_user = ctx.accounts.user.load_mut()?;
        let a_senior = &ctx.accounts.senior;

        let is_senior: bool = a_user.validate_reporter(a_senior.to_account_info().key(), 1)?;
        require!( is_senior,  NewsError::NotSeniorReporter);

        a_pool.state = 3;
        Ok(())
    }

    pub fn publish_news(ctx: Context<PublishNewsContext>) -> Result<()> {
        
        let mut a_pool = ctx.accounts.pool.load_mut()?;
        let mut a_vault = ctx.accounts.vault.load_mut()?;

        let a_user = ctx.accounts.user.load_mut()?;
        let a_admin = &ctx.accounts.admin;

        let is_admin: bool = a_user.validate_reporter(a_admin.to_account_info().key(), 2)?;
        require!( is_admin,  NewsError::NotAdmin);

        require!(
            a_pool.state == 2, 
            NewsError::NotApprovedNews
        );
        
        a_pool.state = 4;
        a_vault.balance += FIXED_SOL;

        Ok(())
    }

    pub fn payout_junior(ctx: Context<PayoutJuniorContext>) -> Result<()> {
        
        {
            let owner_vault = ctx.accounts.owner_vault.load()?;
            let a_vault = ctx.accounts.vault.load()?;

            let a_user = ctx.accounts.user.load_mut()?;
            let a_admin = &ctx.accounts.admin;

            let is_admin: bool = a_user.validate_reporter(a_admin.to_account_info().key(), 2)?;
            require!( is_admin,  NewsError::NotAdmin);

            require!(
                owner_vault.balance > 0 && owner_vault.balance - a_vault.balance > 0, 
                NewsError::InsufficientFunds
            );
            
            require!(
                a_vault.balance > 0, 
                NewsError::NothingRevenue
            );
        }

        {
            let _owner_vault = &mut ctx.accounts.owner_vault;
            let junior = &mut ctx.accounts.junior;

            let from = _owner_vault.to_account_info();
            let to = junior.to_account_info();
            **from.try_borrow_mut_lamports()? -= ctx.accounts.vault.load()?.balance;
            **to.try_borrow_mut_lamports()? += ctx.accounts.vault.load()?.balance;

            let mut a_owner_vault = ctx.accounts.owner_vault.load_mut()?;
            a_owner_vault.balance -= ctx.accounts.vault.load()?.balance;

            let mut a_vault = ctx.accounts.vault.load_mut()?;
            a_vault.balance = 0
        }
        
        Ok(())
    }

    pub fn send_tip(
        ctx: Context<SendTipContext>, 
        price: u64,
    ) -> Result<()> {
        let system_program = &ctx.accounts.system_program;
        let cpi_ctx = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.reporter.to_account_info()
            }
        );

        anchor_lang::system_program::transfer(cpi_ctx, price)?;

        Ok(())
    }

    pub fn send_mint_fee(
        ctx: Context<SendMintFeeContext>, 
        price: u64,
    ) -> Result<()> {
        let system_program = &ctx.accounts.system_program;

        
        let cpi_ctx = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.reporter.to_account_info(),
                to: ctx.accounts.owner.to_account_info()
            }
        );

        anchor_lang::system_program::transfer(cpi_ctx, price)?;

        Ok(())
    }

    pub fn create_campaign(
        ctx: Context<CreateCampaignContext>, 
        campaign_id: u64,
    ) -> Result<()> {
        let mut a_campaign_pool = ctx.accounts.campaign_pool.load_init()?;
        let system_program = &ctx.accounts.system_program;

        let a_advertiser = &ctx.accounts.advertiser;
        let current_time = get_current_time()?;

        a_campaign_pool.campaign_id = campaign_id;
        a_campaign_pool.advertiser = a_advertiser.to_account_info().key();
        a_campaign_pool.created_at = current_time;
        a_campaign_pool.updated_at = current_time;
        a_campaign_pool.state = 0;

        let cpi_ctx = CpiContext::new(
            system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.advertiser.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
            }
        );

        anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        Ok(())
    }

    pub fn edit_campaign(
        ctx: Context<EditCampaignContext>, 
        campaign_id: u64, 
    ) -> Result<()> {
        let mut a_campaign_pool = ctx.accounts.campaign_pool.load_mut()?;
        let current_time = get_current_time()?;

        a_campaign_pool.campaign_id = campaign_id;
        a_campaign_pool.updated_at = current_time;
        a_campaign_pool.state = 1;
        Ok(())
    }

    pub fn delete_campaign(
        ctx: Context<DeleteCampaignContext>
    ) -> Result<()> {
        let a_advertiser = &ctx.accounts.advertiser;
    
        ctx.accounts.campaign_pool.close(a_advertiser.to_account_info())?;
        
        Ok(())
    }

    pub fn approve_campaign(
        ctx: Context<ApproveCampaignContext>
    ) -> Result<()> {
        {
            let a_user = ctx.accounts.user.load_mut()?;
            let a_admin = &ctx.accounts.admin;
    
            let is_admin: bool = a_user.validate_reporter(a_admin.to_account_info().key(), 2)?;
            require!( is_admin,  NewsError::NotAdmin);

            let mut a_campaign_pool = ctx.accounts.campaign_pool.load_mut()?;
            a_campaign_pool.state = 2;
        }
        {
            let system_program = &ctx.accounts.system_program;

             let cpi_ctx = CpiContext::new (
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.admin.to_account_info()
                }
            );
            anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        }

        Ok(())
    }

    pub fn deny_campaign(
        ctx: Context<DenyCampaignContext>
    ) -> Result<()> {
        {   
            let a_user = ctx.accounts.user.load_mut()?;
            let a_admin = &ctx.accounts.admin;
    
            let is_admin: bool = a_user.validate_reporter(a_admin.to_account_info().key(), 2)?;
            require!( is_admin,  NewsError::NotAdmin);

            let mut a_campaign_pool = ctx.accounts.campaign_pool.load_mut()?;
            a_campaign_pool.state = 3;
        }
        {
            let system_program = &ctx.accounts.system_program;

             let cpi_ctx = CpiContext::new (
                system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.advertiser.to_account_info()
                }
            );
            anchor_lang::system_program::transfer(cpi_ctx, FIXED_SOL)?;

        }

        Ok(())
    }
}
