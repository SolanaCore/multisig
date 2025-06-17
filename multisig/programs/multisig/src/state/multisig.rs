use anchor_lang::prelude::*;
use crate::utils::assert_unique_owners::assert_unique_owners;
#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(50)]
    pub owner: Vec<Pubkey>,
    pub threshold: u64,
    pub bump: u8,
}

impl Multisig {
    pub fn set_multisig_details(
        self,
        owners: Vec<Pubkey>,
        threshold: u64,
        bump: u8,
    ) -> Self {
        assert_unique_owners(&owners)?;
        //add some checks for owners and threshold
        require!(
            threshold > 0 && (threshold as usize) <= owners.len(),
            ErrorCode::InvalidThreshold
        );
        Self {
            owner: owners,
            threshold,
            bump,
        }
    }
    pub fn update_threshold(&self, updated_threshold) => Result<(), ErrorCode> {
        assert_gte!(updated_threshold > self.threshold, ErrorCode::InvalidThreshold);
        assert!(updated_threshold <= self.owner.len() as u64, ErrorCode::InvalidThreshold);
        assert_unique_owners(&self.owner)?;
        self.threshold = updated_threshold;
        Ok(())
    }
    
    pub fn owner(&mut self, new_owners: Vec<Pubkey>) -> Result<(), ErrorCode> {
        assert_unique_owners(&new_owners)?;
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