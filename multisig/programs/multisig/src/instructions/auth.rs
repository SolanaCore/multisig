use anchor_lang::prelude::*;
use crate::state::{Multisig};

/// Prevent Client from directly calling this fn()
/// To prevent clients from directly invoking this function, we require the `multisig_signer` (a PDA) to be a signer.
/// Since PDAs have no private keys, only the program itself can sign on their behalf using `invoke_signed`.
/// This ensures that the function can only be called internally via an approved multisig transaction,
/// and not directly from the client side — preserving the integrity of the governance process.
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
