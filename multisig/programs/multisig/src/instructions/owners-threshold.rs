use crate:instructions::auth::Auth;

pub fn change_threshold(ctx: Context<Auth>, new_threshold: u64) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        *multisig.update_threshold(new_threshold)?;
        Ok(())
    }
    pub fn change_owners(ctx: Context<Auth>, new_owners: Vec<Pubkey>) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        *multisig.owner(new_owners.clone())?;

        Ok(())
    }
    pub fn change_owners_and_threshold(
        ctx: Context<Auth>,
        new_owners: Vec<Pubkey>,
        new_threshold: u64,
    ) -> Result<()> {
        assert_unique_owners(&new_owners)?;
        assert!(new_threshold > 0 && new_threshold > new_owners.len() as u64,"{}", ErrorCode::InvalidThreshold);
        change_threshold(ctx, new_threshold)?;
        change_owners(ctx, new_owners)?;
        Ok(())
    }