use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Validate<'info> {
    #[account(mut)]
    pub identity: Account<'info, Identity>,
}

pub fn handler(ctx: Context<Validate>, _tx_hash: [u8; 32]) -> Result<()> {
    ctx.accounts.identity.validated += 1;
    Ok(())
}
