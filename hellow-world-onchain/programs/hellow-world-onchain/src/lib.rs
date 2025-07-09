use anchor_lang::prelude::*;

declare_id!("eybKCTJj9G9W16HXfyYBuVmVpstwa4fRbBEiNxvRAnd");

#[program]
pub mod hellow_world_onchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.greeting = greeting;
        Ok(())
    }
    

    pub fn update(ctx : Context<Update>, new_greeting : String)-> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.greeting = new_greeting;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init , 
        seeds = [b"greeting", user.key().as_ref()], bump,
        payer = user, 
        space = 8+64
    )]
    pub base_account : Account<'info, GreetingAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>,

}

#[derive(Accounts)]
pub struct Update<'info>{
    // this only needs to derive the pda and the user account
    #[account(
        mut, 
        seeds = [b"greeting",authority.key().as_ref()], bump
    )]
    pub base_account : Account<'info, GreetingAccount>,
    pub authority : Signer<'info>,
}

#[account]
pub struct GreetingAccount {
    pub greeting: String,
}
