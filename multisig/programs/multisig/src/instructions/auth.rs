use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
use crate::constants::SEED;
#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut,
        seeds = [SEED.to_le_bytes(), multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: Signer<'info>,
}
