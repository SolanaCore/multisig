use anchor_lang::prelude::*;
use crate::state::{Multisig, Transaction};
// use crate::constants::SEED;

    #[derive(Accounts)]
    pub struct CloseTransaction<'info> {
        pub multisig: Box<Account<'info, Multisig>>,

        #[account(
            mut,
            close = proposer, // Close the transaction account and transfer remaining SOL to the multisig account
            has_one = multisig
        )]
        pub transaction: Box<Account<'info, Transaction>>,

        #[account(mut)]
        pub proposer: Signer<'info>,
    }

#[event]
pub struct TransactionClosed {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
}

pub fn close_transaction(ctx: Context<CloseTransaction>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;

    transaction.validate(multisig)?;
    transaction.close_tx(proposer.key())?;
    // Emit an event for the cancelled transaction
    emit!(TransactionClosed {
        multisig: multisig.key(),
        transaction: transaction.key(),
        program_id: *ctx.program_id,
    });

    Ok(())
}