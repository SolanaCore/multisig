// programs/multisig/src/lib.rs

use anchor_lang::prelude::*;

#[allow(unused)]
pub mod constants;
pub mod error;
pub mod instructions; // This line already re-exports the modules, but direct calls are better within the program
pub mod state;
pub mod utils;

#[allow(unused_imports)]
use instructions::*; // Keep this to bring the context structs into scope
use state::*;
#[allow(unused_imports)]
use error::ErrorCode;
#[allow(unused_imports)]
use utils::*;


#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "multisig",
    project_url: "https://github.com/SolanaCore/multisig",
    contacts:"artificialintelligencehub35@gmail.com",
    policy: "https://github.com/SolanaCore/multisig/blob/main/SECURITY.MD",
    source_code: "https://github.com/SolanaCore/multisig",
    preferred_languages: "en"
}

declare_id!("C9h86YyYMpKViSzRpE7XUPrRVRypu5WTSitJ9n8czcZh");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod solana_core_multisig {
    use super::*;

    pub fn initialize_multisig(
        ctx: Context<InitMultisig>, owners: Vec<Pubkey>, threshold: u64, bump: u8,
    ) -> Result<()> {
        // Direct call to the function imported by `use instructions::*;` or simply define it here
        instructions::init_multisig(ctx, owners, threshold, bump)
    }

    pub fn create_tx(
        ctx: Context<InitTransaction>, pid: Pubkey, data: Vec<u8>, accs: Vec<TransactionAccount>
    ) -> Result<()> {
        instructions::init_transaction(ctx, &pid, accs, data)
    }

    pub fn execute_tx(ctx: Context<ExecuteTransaction>) -> Result<()> { // Removed &mut as it's typically Context<T>
        instructions::execute_transaction(ctx)
    }

    pub fn edit_tx(ctx: Context<EditTransaction>, data: Vec<u8>, accs: Vec<TransactionAccount>) -> Result<()> {
        instructions::edit_transaction(ctx, accs, data)
    }

    pub fn cancel_tx(ctx: Context<CancelTransaction>) -> Result<()> {
        instructions::cancel_transaction(ctx)
    }

    pub fn revoke_approval(ctx: Context<RevokeApproval>) -> Result<()> {
        instructions::revoke_approval(ctx)
    }

    pub fn approve(ctx: Context<ApproveTransaction>) -> Result<()> {
        instructions::approve_transaction(ctx)
    }
}
