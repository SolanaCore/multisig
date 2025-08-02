use anchor_lang::prelude::*;
use crate::{
    instructions::Auth,
    error::ErrorCode,
};

    pub fn change_threshold(ctx: &mut Context<Auth>, new_threshold: u64) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.update_threshold(new_threshold)?;
        Ok(())
    }
    
    pub fn change_owners(ctx: &mut Context<Auth>, new_owners: Vec<Pubkey>) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        multisig.owner(new_owners.clone())?;

        Ok(())
    }
    pub fn change_owners_and_threshold(
        mut ctx: Context<Auth>,
        new_owners: Vec<Pubkey>,
        new_threshold: u64,
    ) -> Result<()> {
        // assert_unique_owners(&new_owners)?;
        assert!(new_threshold > 0 && new_threshold > new_owners.len() as u64,"{}", ErrorCode::InvalidThreshold);
        change_threshold(&mut ctx, new_threshold)?;
        change_owners(&mut ctx, new_owners.clone())?;

        emit!(ChangedOwnerAndThreshold {
        multisig: ctx.accounts.multisig.clone().key(),
        new_owners: new_owners,
        new_threshold: new_threshold,
        });

        Ok(())
    }

    //event
    #[event(discriminator = [9])]
    pub struct ChangedOwnerAndThreshold{
        multisig: Pubkey,
        new_owners: Vec<Pubkey>,
        new_threshold: u64
    }