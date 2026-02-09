use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub identity: Account<'info, Identity>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Register>) -> Result<()> {
    ctx.accounts.identity.owner = ctx.accounts.user.key();
    ctx.accounts.identity.validated = 0;
    Ok(())
}
