use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::prelude::Signer;
use crate::state::{Multisig, Transaction, TransactionAccount};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct EditTransaction<'info> {

    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    /// CHECK: multisig_signer is a PDA that is used to sign transactions on behalf of the multisig
    #[account(
        mut,
        seeds = [b"multisig", multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: AccountInfo<'info>,

    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,

    pub proposer: Signer<'info>,
}

#[event]
pub struct TransactionEdited {
    pub multisig: Pubkey,
    pub transaction: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<TransactionAccount>,
    pub data: Vec<u8>,
    pub signers: Vec<bool>,
}

pub fn edit_transaction(
    ctx: Context<EditTransaction>,
    accs: Vec<TransactionAccount>,
    data: Vec<u8>,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;
    let proposer = &ctx.accounts.proposer;

    // Ensure the proposer is one of the owners of the multisig
    if !multisig.owner.contains(&proposer.key()) {
        return Err(ErrorCode::InvalidOwner.into());
    }

    transaction.check_if_already_executed()?;
    transaction.edit_tx(
        accs,
        data,
        proposer.key(),
    )?;

    // Emit an event for the edited transaction
    emit!(TransactionEdited {
        multisig: multisig.key(),
        transaction: transaction.key(),
        program_id: *ctx.program_id,
        accounts: transaction.accounts.clone(),
        data: transaction.data.clone(),
        signers: transaction.signers.clone(),
    });

    Ok(())
}