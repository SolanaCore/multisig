use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct RevokeApproval<'info>{
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,

    pub proposer: Signer<'info>,
}

pub fn revoke_approval(ctx: Context<RevokeApproval>) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;

    // Ensure the proposer is one of the owners of the multisig
    if !multisig.owner.contains(&proposer.key()) {
        return Err(ErrorCode::InvalidOwner.into());
    }
    transaction.check_if_already_executed()?;
    transaction.revoke_approval(multisig, proposer.key())?;
    // Remove the proposer's approval from the transaction

    // Emit an event for the revoked approval
    emit!(TransactionRevoked {
        multisig: multisig.key(),
        transaction: transaction.key(),
        proposer: proposer.key(),
    });

    Ok(())
}

#[event]
pub struct TransactionRevoked {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub proposer: Pubkey,
}