fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
    let mut seen = HashSet::new();
    for owner in owners {
        if !seen.insert(owner) {
            return Err(error!(ErrorCode::InvalidThreshold));
        }
    }
    Ok(())
}