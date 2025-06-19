use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
// use crate::constants::SEED;

    #[derive(Accounts)]
    pub struct CancelTransaction<'info> {
        pub multisig: Box<Account<'info, Multisig>>,

        #[account(
            mut,
            close = proposer, // Close the transaction account and transfer remaining SOL to the multisig account
        )]
        pub transaction: Box<Account<'info, Transaction>>,

        #[account(mut)]
        pub proposer: Signer<'info>,
    }

#[event]
pub struct TransactionCancelled {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
}

pub fn cancel_transaction(ctx: Context<CancelTransaction>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;

    transaction.check_if_already_executed()?;
    transaction.validate(multisig)?;
    transaction.cancel(proposer.key())?;
    // Emit an event for the cancelled transaction
    emit!(TransactionCancelled {
        multisig: multisig.key(),
        transaction: transaction.key(),
        program_id: *ctx.program_id,
    });

    Ok(())
}