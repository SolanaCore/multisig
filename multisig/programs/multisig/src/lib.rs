#[allow(unused)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

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
    preferred_languages: "en,de",
declare_id!("AwhGP9QqsN2JAaS2XyYo2PeC2EAvkCExYLd5Mfuq1GaQ");

#[program]
pub mod multisig {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
