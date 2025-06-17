use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use std::collections::HashSet;
use std::ops::Deref;
use anchor_lang::solana_program::program::invoke_signed;

#[allow(unused)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use utils::assert_unique_owners;
pub use constants::SEED;
pub use instructions::{
    approve::Approve,
    auth::Auth,
    create_multisig::CreateMultisig,
    create_transaction::CreateTransaction,
    exec_tx::ExecuteTransaction,
    change_owners::ChangeOwners,
    change_threshold::ChangeThreshold,
    edit_tx::EditTransaction,
    cancel_tx::CancelTransaction,
    initialize_multisig::InitializeMultisig,
    approve::Approve,
    change_threshold::ChangeThreshold,
    change_owners::ChangeOwners,
};
pub use state::{Multisig, Transaction, TransactionAccount};
pub use error::ErrorCode;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "multisig",
    project_url: "https://github.com/SolanaCore/multisig",
    contacts:"mailto:artificialintelligencehub35@gmail.com",
    policy: "https://github.com/SolanaCore/multisig/blob/main/SECURITY.MD",
    source_code: "https://github.com/SolanaCore/multisig",
    preferred_languages: "en"
}


declare_id!("AwhGP9QqsN2JAaS2XyYo2PeC2EAvkCExYLd5Mfuq1GaQ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod SolanaCoreMultisig {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u64,
        bump: u8,
    ) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.set_multisig_details(
            owners,
            threshold,
            bump,
        )?;
        Ok(())
    }

    pub fn create_tx(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        data: Vec<u8>,
        accs: Vec<TransactionAccount>,
    ) -> Result<()> {
        tx.set_tx_details(
            &ctx.accounts.multisig.key(),
            &pid,
            accs,
            data,
        )?;
       
        Ok(())
    }

    pub fn execute_tx(ctx: Context<ExecTx>) -> Result<()> {
        let multisig_key = ctx.accounts.multisig.key();
        let tx = &mut ctx.accounts.tx;
        tx.validate(&ctx.accounts.multisig_signer)?;
        tx.check_if_already_executed(&ctx.accounts.multisig)?;
        tx.format_ix(multisig_key)?;
        /*
        ctx.accounts.multisig.key().as_ref(),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
             &[ctx.accounts.multisig.bump],
         ];
         */
        let seeds = [SEED.to_le_bytes(), multisig_key.as_ref(), &[ctx.accounts.multisig.bump]];
        let signer_seeds = &[&seeds[..]];
        let rem_accs = ctx.remaining_accounts;
        invoke_signed(&ix, rem_accs, signer_seeds)?;
        tx.did_execute()?;
        Ok(())
    }
    pub fn edit_tx(Context<EditTransaction>, pid: Pubkey, data: Vec<u8>, accs: Vec<TransactionAccount>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        tx.edit_tx_details(&pid, accs, data)?;
        Ok(())
    }
    pub fn cancel_tx(ctx: Context<CancelTransaction>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        tx.cancel()?;
        Ok(())
    }
    pub fn approve(ctx: Context<Approve>) -> Result<(), ErrorCode> {
        //verify if this person is there in the owners list of the multisig pda
        let tx  = &mut ctx.accounts.transaction;
        tx.approve(ctx.accounts.signer.key)?;
        Ok(())
    }
    
    pub fn change_threshold(ctx: Context<Auth>, new_threshold: u64) -> Result<(), ErrorCode> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.update_threshold(new_threshold)?;
        Ok(())
    }
    pub fn change_owners(ctx: Context<Auth>, new_owners: Vec<Pubkey>) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.owner(new_owners.clone())?;

        Ok(())
    }
}
