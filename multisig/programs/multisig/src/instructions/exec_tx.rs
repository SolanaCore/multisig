use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
use crate::constants::SEED;
// #[derive(Accounts)]
// pub struct ExecuteTransaction<'info> {
//     #[account(mut, signer)]
//     pub multisig: Box<Account<'info, Multisig>>,
    
//     /// CHECK: This PDA is to verify
//     #[account(
//         seeds = [bSEED, multisig.key().as_ref()],
//         bump = multisig.bump
//     )]
//     pub multisig_signer: Box<AccountInfo<'info>>,
//     #[account(mut)]
//     pub tx: Box<Account<'info, Transaction>>,
// }

#[derive(Account)]
pub struct ExecuteTransaction {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut, 
        seeds = [SEED.to_le_bytes(), multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: Box<AccountInfo<'info>>,

    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,

    pub proposer: Signer<'info>,
}

#[event]
pub struct TransactionExecuted {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
}