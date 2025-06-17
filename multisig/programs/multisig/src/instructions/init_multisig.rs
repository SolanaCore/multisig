use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};

#[derive(Accounts)]
pub struct InitMultisig<'info> {
    #[account(zero, signer)]
    pub multisig: Box<Account<'info, Multisig>>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct MultisigInitialized {
    pub multisig: Pubkey,
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8, 
}
