use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;

#[allow(unused)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

// use utils::assert_unique_owner;
use instructions::*;
use state::*;
use error::ErrorCode;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "multisig",
    project_url: "https://github.com/SolanaCore/multisig",
    contacts:"mailto:artificialintelligencehub35@gmail.com",
    policy: "https://github.com/SolanaCore/multisig/blob/main/SECURITY.MD",
    source_code: "https://github.com/SolanaCore/multisig",
    preferred_languages: "en"
}

//pub type Result<T> = std::result::Result<T, anchor_lang::prelude::Error>;


declare_id!("AwhGP9QqsN2JAaS2XyYo2PeC2EAvkCExYLd5Mfuq1GaQ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod SolanaCoreMultisig {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<CreateMultisig>, owners: Vec<Pubkey>, threshold: u64, bump: u8,
    ) -> Result<()> {
        instruction::init_multisig(Context<'_', '_', '_', '_', CreateMultisig<'_'>>, owners, threshold, bump)
    }

    pub fn create_tx(
        ctx: Context<InitTransaction>, pid: Pubkey, data: Vec<u8>, accs: Vec<TransactionAccount>
    ) -> Result<()> {
        instruction::init_transaction(Context<'_', '_', '_', '_', InitTransaction<'_'>>, pid, data, accs)
    }

    pub fn execute_tx(ctx: Context<ExecuteTransaction>) -> Result<()> {
        instruction::execute_transaction(Context<'_', '_', '_', '_', ExecuteTransaction<'_'>>) 
    }

    pub fn edit_tx(ctx: Context<EditTransaction>, pid: Pubkey, data: Vec<u8>, accs: Vec<TransactionAccount>) -> Result<()> {
        instruction::edit_transaction(Context<'_', '_', '_', '_', EditTransaction<'_'>>, pid, data, accs)
    }

    pub fn cancel_tx(ctx: Context<CancelTransaction>) -> Result<()> {
        instruction::cancel_transaction(Context<'_', '_', '_', '_', CancelTransaction<'_'>>)
    }

    pub fn revoke_approval(ctx: Context<RevokeApproval>) -> Result<()> {
        instruction::revoke_approval(Context<'_', '_', '_', '_', RevokeApproval<'_'>>)
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        instruction::approve(Context<'_', '_', '_', '_', Approve<'_'>>)
    }
    
}
