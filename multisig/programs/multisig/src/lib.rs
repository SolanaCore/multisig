use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use std::collections::HashSet;

#[allow(unused)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;


pub use constants::*;
pub use instructions::*;
pub use state::*;

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
        require!(
            threshold > 0 && (threshold as usize) <= owners.len(),
            ErrorCode::InvalidThreshold
        );
        assert_unique_owners(&owners)?;

        let multisig = &mut ctx.accounts.multisig;
        multisig.bump = bump;
        multisig.owner = owners;
        multisig.threshold = threshold;

        Ok(())
    }

    pub fn create_tx(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        data: Vec<u8>,
        accs: Vec<TransactionAccount>,
    ) -> Result<()> {
        let owner_index = ctx
            .accounts
            .multisig
            .owner
            .iter()
            .position(|a| a == ctx.accounts.proposer.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = vec![false; ctx.accounts.multisig.owner.len()];
        signers[owner_index] = true;

        let tx = &mut ctx.accounts.transaction;
        tx.program_id = pid;
        tx.accounts = accs;
        tx.data = data;
        tx.signers = signers;
        tx.multisig = ctx.accounts.multisig.key();
        tx.did_execute = false;
        Ok(())
    }

    pub fn execute_tx(ctx: Context<ExecTx>) -> Result<()> {
        require!(ctx.accounts.tx.did_execute == false, ErrorCode::TxExecuted);
        let approval_count = ctx.accounts.tx.signers.iter().filter(|&&b| b).count() as u64;
        let threshold = ctx.accounts.multisig.threshold;
        require!(approval_count >= threshold, ErrorCode::InsufficientSigners);
        let mut ix = (*ctx.accounts.tx.deref()).to_instruction();
        ix.accounts = ix
            .accounts
            .iter()
            .map(|acc| {
                let mut acc = acc.clone();
                if acc.pubkey == *ctx.accounts.multisig_signer.key {
                    acc.is_signer = true;
                }
                acc
            }).collect();

        let multisig_key = ctx.accounts.multisig.key();
        /*
        ctx.accounts.multisig.key().as_ref(),
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
             &[ctx.accounts.multisig.bump],
         ];
         */
        let seeds = [multisig_key.as_ref(), &[ctx.accounts.multisig.bump]];
        let signer_seeds = &[&seeds[..]];
        let rem_accs = ctx.remaining_accounts;
        invoke_signed(&ix, rem_accs, signer_seeds)?;
        ctx.accounts.tx.did_execute = true;
        Ok(())
    }
    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        //verify if this person is there in the owners list of the multisig pda
        let signer = ctx.accounts.signer.key;
        let owner_index = ctx
            .accounts
            .multisig
            .owner
            .iter()
            .position(|a| a == signer)
            .ok_or(ErrorCode::InvalidOwner)?;
        ctx.accounts.transaction.signers[owner_index] = true;
        Ok(())
    }
    pub fn change_threshold(ctx: Context<Auth>, new_threshold: u64) -> Result<()> {
        if new_threshold < 0
            && new_threshold > ctx.accounts.multisig.owner.len().try_into().unwrap()
        {
            return err!(ErrorCode::InvalidThreshold);
        }
        let multisig = &mut ctx.accounts.multisig;
        multisig.threshold = new_threshold;
        Ok(())
    }
    pub fn change_owners(ctx: Context<Auth>, new_owners: Vec<Pubkey>) -> Result<()> {
        if new_owners.len() < ctx.accounts.multisig.threshold.try_into().unwrap()
            && new_owners.len() == 0
        {
            return err!(ErrorCode::InvalidOwner);
        }
        assert_unique_owners(&new_owners)?;
        let multisig = &mut ctx.accounts.multisig;
        multisig.owner = new_owners;
        Ok(())
    }
    //todo

    pub fn reject_transaction() {

    }
    pub fn cancel_transaction() {

    }
    pub fn edit_tx(){

    }
}

// ---------- Accounts ----------
#[derive(Accounts)]
pub struct ExecTx<'info> {
    #[account(mut, signer)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        seeds = [multisig.key().as_ref()],
        bump = multisig.bump
    )]
    pub multisig_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub tx: Box<Account<'info, Transaction>>,
}

#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(zero, signer)]
    pub multisig: Box<Account<'info, Multisig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,
    #[account(zero)]
    pub transaction: Box<Account<'info, Transaction>>,
    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    signer: Signer<'info>,
    multisig: Box<Account<'info, Multisig>>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
}
#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut,
        seeds = [multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: Signer<'info>,
}

// ---------- Account Structs ----------

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
    #[max_len(25)]
    pub accounts: Vec<TransactionAccount>,
    #[max_len(1024)]
    pub data: Vec<u8>,
    #[max_len(25)]
    pub signers: Vec<bool>,
    pub did_execute: bool,
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(50)]
    pub owner: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}

// ---------- Utility ----------

fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
    let mut seen = HashSet::new();
    for owner in owners {
        if !seen.insert(owner) {
            return Err(error!(ErrorCode::InvalidThreshold));
        }
    }
    Ok(())
}
//IMP-> Rust's orphan rule forbids this: you can only implement foreign traits for local types, or local traits for foreign types.
pub trait ToIx {
    fn to_instruction(&self) -> Instruction;
}
impl<'info> ToIx for Account<'info, Transaction> {
    fn to_instruction(&self) -> Instruction {
        Instruction {
            program_id: self.program_id,
            accounts: self
                .accounts
                .iter()
                .map(|acc| AccountMeta {
                    pubkey: acc.pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                }).collect(),
            data: self.data.clone(),
        }
    }
}

