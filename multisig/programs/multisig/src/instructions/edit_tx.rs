use crate::state::{Multisig, Transaction};
use crate::constants::SEED;

#[derive(Accounts)]
pub struct EditTransaction<'info> {

    #[account(mut)]
    pub multisig: Box<Account<'info, Multisig>>,

    #[account(
        mut,
        seeds = [SEED.to_le_bytes(), multisig.key().as_ref()],
        bump = multisig.bump,
    )]
    pub multisig_signer: Signer<'info>,

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