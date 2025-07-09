use anchor_lang::prelude::*;

declare_id!("8EWr8AC7GmTpzx8kCH2FTqYYBQ5HT54xGNDyohspfCiC");

#[program]
pub mod counter_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        ctx.accounts.counter.authority = ctx.accounts.user.key();
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateCounter>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.counter.authority,
            ctx.accounts.authority.key(),
            CustomError::Unauthorized
        );
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<UpdateCounter>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.counter.authority,
            ctx.accounts.authority.key(),
            CustomError::Unauthorized
        );
        ctx.accounts.counter.count -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
}

// require_keys_eq! -> Does the signer (authority) match the authority saved in the counter account?
