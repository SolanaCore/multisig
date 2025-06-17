use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction, TransactionAccount};
use crate::error::ErrorCode;
// #[derive(Accounts)]
// pub struct CreateTransaction<'info> {
//     #[account(mut)]
//     pub multisig: Box<Account<'info, Multisig>>,
//     #[account(zero)]
//     pub transaction: Box<Account<'info, Transaction>>,
//     pub proposer: Signer<'info>,
// }


#[derive(Accounts)]
pub struct InitTransaction<'info>{
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

pub fn init_transaction(ctx: Context<InitTransaction>, accounts: Vec<TransactionAccount>, data: Vec<u8>, signers: Vec<bool>) -> Result<()> {
    let multisig = ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;

    // Ensure the proposer is one of the owners of the multisig
    if !multisig.owner.contains(&proposer.key()) {
        return Err(ErrorCode::InvalidOwner.into());
    }
    transaction.init(
        multisig,
        &proposer.key(),
        accounts,
        data,
        proposer,
    )?;
    
    // Emit an event for the created transaction
    emit!(TransactionCreated {
        multisig: multisig.key(),
        program_id: *ctx.program_id,
        accounts: transaction.accounts.clone(),
        data: transaction.data.clone(),
        signers: transaction.signers.clone(),
    });

    Ok(())
}