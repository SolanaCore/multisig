use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};


#[derive(Accounts)]
pub struct RevokeApproval<'info>{
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,

    pub proposer: Signer<'info>,
}