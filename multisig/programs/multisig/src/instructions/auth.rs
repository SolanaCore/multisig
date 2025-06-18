use anchor_lang::prelude::*;
use crate::state::{Multisig};
#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut,
        seeds = [b"multisig", multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: Signer<'info>,
}

#[event]
pub struct AuthEvent {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
}
