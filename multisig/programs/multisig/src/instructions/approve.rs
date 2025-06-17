use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
// #[derive(Accounts)]
// pub struct Approve<'info> {
//     signer: Signer<'info>,
//     multisig: Box<Account<'info, Multisig>>,
//     #[account(mut, has_one = multisig)]
//     transaction: Box<Account<'info, Transaction>>,
// }

#[derive(Account)]
pub struct ApproveTransaction{
    pub multisig: Box<Account<'info, Multisig>>,
    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,
    pub proposer: Signer<'info>,
}


#[event]
pub struct TransactionApproved {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
    pub approver: Pubkey,
}
