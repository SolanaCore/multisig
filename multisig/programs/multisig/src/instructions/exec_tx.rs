use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::prelude::Context;
use anchor_lang::prelude::Pubkey;
use crate::state::{Multisig, Transaction, TransactionAccount};
use crate::error::ErrorCode;
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

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut, signer)]
    pub multisig: Box<Account<'info, Multisig>>,

   /// CHECK: multisig_signer is a PDA program signer. Data is never read or written to
    #[account(
        seeds = [b"multisig", multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    multisig_signer: UncheckedAccount<'info>,

    #[account(mut)]
    pub transaction: Box<Account<'info, Transaction>>,
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

pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    let transaction = &mut ctx.accounts.transaction;

    // Check if the transaction is already executed
    if transaction.did_execute {
        return Err(ErrorCode::TransactionAlreadyExecuted.into());
    }

    let multisig_key = ctx.accounts.multisig.key();
        transaction.validate(&ctx.accounts.multisig)?;
        transaction.check_if_already_executed()?;
        let ix: Instruction = transaction.format_ix(&ctx.accounts.multisig_signer.key());
        /*
        ctx.accounts.multisig.key().as_ref(),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
             &[ctx.accounts.multisig.bump],
         ];
         */
        let seeds = [b"multisig", multisig_key.as_ref(), &[ctx.accounts.multisig.bump]];
        let signer_seeds = &[&seeds[..]];
        let rem_accs = ctx.remaining_accounts;
        invoke_signed(&ix, rem_accs, signer_seeds)?;
        transaction.did_execute()?;
    
    // Emit an event for the executed transaction
    emit!(TransactionExecuted {
        multisig: multisig.key(),
        transaction: transaction.key(),
        program_id: *ctx.program_id,
        accounts: transaction.accounts.clone(),
        data: transaction.data.clone(),
        signers: transaction.signers.clone(),
    });

    Ok(())
}