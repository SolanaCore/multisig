use anchor_lang::prelude::*;
use crate::state::{Multisig};

#[derive(Accounts)]
pub struct InitMultisig<'info> {
    #[account(zero, signer)]
    pub multisig: Box<Account<'info, Multisig>>,
}

#[event]
pub struct MultisigInitialized {
    pub multisig: Pubkey,
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8, 
}

pub fn init_multisig(
    ctx: Context<InitMultisig>,
    owners: Vec<Pubkey>,
    threshold: u64,
    bump: u8,
) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;

    multisig.init(owners, threshold, bump)?;

    // Emit an event for the initialized multisig
    emit!(MultisigInitialized {
        multisig: multisig.key(),
        owners: multisig.owner.clone(),
        threshold: multisig.threshold,
        bump: multisig.bump,
    });

    Ok(())
}