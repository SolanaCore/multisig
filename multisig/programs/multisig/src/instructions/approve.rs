use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
use crate::error::ErrorCode;
// #[derive(Accounts)]
// pub struct Approve<'info> {
//     signer: Signer<'info>,
//     multisig: Box<Account<'info, Multisig>>,
//     #[account(mut, has_one = multisig)]
//     transaction: Box<Account<'info, Transaction>>,
// }

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    pub multisig: Box<Account<'info, Multisig>>,
    #[account(mut, has_one = multisig)]
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

pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;
    //check if the proper exist in the multisig's owner list
    multisig.owner.contains(&proposer.key())
        .then_some(())
        .ok_or(ErrorCode::InvalidOwner)?;

    transaction.approve(proposer.key(), multisig)?;

    // Emit an event for the approved transaction
    emit!(TransactionApproved {
        multisig: multisig.key(),
        transaction: transaction.key(),
        program_id: *ctx.program_id,
        approver: proposer.key(),
    });

    Ok(())
}
