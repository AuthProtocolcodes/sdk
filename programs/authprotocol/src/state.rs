use anchor_lang::prelude::*;

#[account]
pub struct Identity {
    pub owner: Pubkey,
    pub validated: u64,
}
