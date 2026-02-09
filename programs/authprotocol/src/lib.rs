use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("Auth1111111111111111111111111111111111");

#[program]
pub mod authprotocol {
    use super::*;

    pub fn register_identity(ctx: Context<Register>) -> Result<()> {
        instructions::register::handler(ctx)
    }

    pub fn validate_tx(ctx: Context<Validate>, tx_hash: [u8; 32]) -> Result<()> {
        instructions::validate::handler(ctx, tx_hash)
    }

    pub fn reward_validator(ctx: Context<Reward>, amount: u64) -> Result<()> {
        instructions::reward::handler(ctx, amount)
    }
}
