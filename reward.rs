use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Reward<'info> {
    pub validator: Signer<'info>,
}

pub fn handler(_ctx: Context<Reward>, _amount: u64) -> Result<()> {
    Ok(())
}
