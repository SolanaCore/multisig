use anchor_lang::prelude::*;
use crate::utils::assert_unique_owner::*;
use crate::error::ErrorCode;

#[account]
#[derive(InitSpace, Eq,PartialEq, Debug)]
pub struct Multisig {
    #[max_len(50)]
    pub owner: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}

impl Multisig {
    pub fn init(
        &mut self,
        owners: Vec<Pubkey>,
        threshold: u64,
        bump: u8,
    ) -> Result<()> {
        assert_unique_owner(&owners)?;
        //add some checks for owners and threshold
        require!(
            threshold > 0 && (threshold as usize) <= owners.len(),
            ErrorCode::InvalidThreshold
        );
        self.owner = owners;
        self.threshold = threshold;
        self.bump = bump;
        Ok(())
    }

    pub fn update_threshold(&mut self, updated_threshold:u64) -> Result<()> {
        assert!(updated_threshold > self.threshold,"{}", ErrorCode::InvalidThreshold);
        assert!(updated_threshold <= self.owner.len() as u64,"{}", ErrorCode::InvalidThreshold);
        assert_unique_owner(&self.owner)?;
        self.threshold = updated_threshold;
        Ok(())
    }
    
    pub fn owner(&mut self, new_owners: Vec<Pubkey>) -> Result<()> {
        assert_unique_owner(&new_owners)?;
        require!(
            new_owners.len() >= self.threshold as usize,
            ErrorCode::InvalidOwner
        );
        self.owner = new_owners;
        Ok(())
    }
}

#[event]
pub struct MultisigInitialized {
    pub multisig: Pubkey,
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}