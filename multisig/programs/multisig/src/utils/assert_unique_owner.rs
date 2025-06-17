use std::collections::HashSet;
use anchor_lang::prelude::Pubkey;
use crate::error::ErrorCode;
pub fn assert_unique_owner(owners: &[Pubkey]) -> Result<(), ErrorCode> {
    let mut seen = HashSet::new();
    for owner in owners {
        if !seen.insert(owner) {
            return Err(ErrorCode::InvalidThreshold.into());
        }
    }
    Ok(())
}