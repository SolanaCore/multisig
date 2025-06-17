use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};

// #[derive(Accounts)]
// pub struct CreateTransaction<'info> {
//     #[account(mut)]
//     pub multisig: Box<Account<'info, Multisig>>,
//     #[account(zero)]
//     pub transaction: Box<Account<'info, Transaction>>,
//     pub proposer: Signer<'info>,
// }


#[derive(Account)]
pub struct InitTransaction {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(zero)]
    pub transaction: Box<Account<'info, Transaction>>,

    pub proposer: Signer<'info>,
}


#[event]
pub struct TransactionCreated {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
}
