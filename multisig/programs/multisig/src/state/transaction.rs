use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use crate::error::ErrorCode;
use crate::state::{Multisig};

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    pub program_id: Pubkey,
    #[max_len(25)]
    pub accounts: Vec<TransactionAccount>,
    #[max_len(1024)]
    pub data: Vec<u8>,
    #[max_len(25)]
    pub signers: Vec<bool>,
    pub did_execute: bool,
    pub owner: Pubkey
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}


impl<'info> Transaction{
        pub fn init(
        &mut self,
        multisig: &Box<Account<Multisig>>,
        program_id: &Pubkey,
        accounts: Vec<TransactionAccount>,
        data: Vec<u8>,
        proposer: Pubkey,
    ) -> Result<()> {
        let proposer_index = multisig.owner.iter().position(|a| *a == proposer.key()).ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = vec![false; multisig.owner.len()];
        signers[proposer_index] = true;

        self.multisig = multisig.clone().key(); // assuming multisig is an account
        self.program_id = *program_id;
        self.accounts = accounts;
        self.data = data;
        self.signers = signers;
        self.did_execute = false;
        self.owner = proposer.key();

        Ok(())
    }


    pub fn approve(&mut self, signer:Pubkey, multisig:&Multisig) -> Result<()> {
        let owner_index = multisig
            .owner
            .iter()
            .position(|a| *a == signer)
            .ok_or(ErrorCode::InvalidOwner)?;
        self.signers[owner_index] = true;
        Ok(())
    }

    pub fn validate(&self, multisig: &Multisig) -> Result<()> {
        require!(self.did_execute == false, ErrorCode::TransactionAlreadyExecuted);
        let approval_count = self.signers.iter().filter(|&&b| b).count() as u64;
        let threshold = multisig.threshold;
        require!(approval_count >= threshold, ErrorCode::InsufficientSigners);
        Ok(())
    }
    pub fn check_if_already_executed(&self) -> Result<()> {
        let did_execute = self.did_execute;
        require!(did_execute == true, ErrorCode::TransactionAlreadyExecuted);
        Ok(())
    }
    pub fn format_ix(&self, multisig_signer:&Pubkey) -> Instruction {
        //first build the ix 
        let mut ix = (*self).to_instruction();
        ix.accounts = ix
            .accounts
            .iter()
            .map(|acc| {
                let mut acc = acc.clone();
                if acc.pubkey == *multisig_signer {
                    acc.is_signer = true;
                }
                acc
            }).collect();
            ix
    }
    pub fn did_execute(&mut self) -> Result<()> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted.into());
        }
        self.did_execute = true;
        Ok(())
    }

    pub fn edit_tx(
        &mut self,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
        proposer: Pubkey,
    ) -> Result<()> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted.into());
        }
        if accs.is_empty() || data.is_empty() {
            return Err(ErrorCode::InvalidTransactionDetails.into());
        }

        if proposer == self.owner {
        self.accounts = accs;
        self.data = data;
        }else {
            return Err(ErrorCode::InvalidOwner.into());
        }
        Ok(())
    }

    pub fn cancel(&self, proposer:Pubkey) -> Result<()> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted.into());
        }
        assert!(self.owner == proposer,"{}", ErrorCode::InvalidOwner);
        Ok(())
    }

    pub fn revoke_approval(&mut self, multisig:&Multisig, signer: Pubkey) -> Result<()> {
        let owner_index = multisig
            .owner
            .iter()
            .position(|a| a == &signer)
            .ok_or(ErrorCode::InvalidOwner)?;
        self.signers[owner_index] = false;
        Ok(())
    }
}

pub trait ToIx {
    fn to_instruction(&self) -> Instruction;
}

impl<'info> ToIx for Transaction {
    fn to_instruction(&self) -> Instruction {
        Instruction {
            program_id: self.program_id,
            accounts: self
                .accounts
                .iter()
                .map(|acc| AccountMeta {
                    pubkey: acc.pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                }).collect(),
            data: self.data.clone(),
        }
    }
}


