use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
use crate::constants::SEED;
pub struct CancelTransaction<'info> {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut,
        seeds = [SEED.to_le_bytes(), multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_pda: Box<AccountInfo<'info>>,

    #[account(mut,
        close = proposer, // Close the transaction account and transfer remaining SOL to the multisig account
    )]
    pub transaction: Box<Account<'info, Transaction>>,

    #[account(signer)]
    pub proposer: Signer<'info>,
}

#[event]
pub struct TransactionCancelled {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
}