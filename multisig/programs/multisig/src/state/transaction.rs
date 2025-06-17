use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use crate::errors::ErrorCode;
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

impl Transaction{
        pub fn init(
        &mut self,
        ctx: &Context<CreateTransaction>, // <-- pass context explicitly
        multisig: Multisig,
        program_id: &Pubkey,
        accounts: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        let proposer_index = ctx
            .accounts
            .multisig
            .owner
            .iter()
            .position(|a| a == ctx.accounts.proposer.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = vec![false; ctx.accounts.multisig.owner.len()];
        signers[proposer_index] = true;

        self.multisig = *multisig.key(); // assuming multisig is an account
        self.program_id = *program_id;
        self.accounts = accounts;
        self.data = data;
        self.signers = signers;
        self.did_execute = false;

        Ok(())
    }


    pub fn approve(self, signer) -> Result<(), ErrorCode::InvalidOwner> {
        let owner_index = ctx
            .accounts
            .multisig
            .owner
            .iter()
            .position(|a| a == signer)
            .ok_or(ErrorCode::InvalidOwner)?;
        ctx.accounts.transaction.signers[owner_index] = true;
        Ok(())
    }

    pub fn validate(&self, multisig: &Multisig) -> Result<(), ErrorCode> {
        require!(self.did_execute == false, ErrorCode::TransactionAlreadyExecuted);
        let approval_count = self.signers.iter().filter(|&&b| b).count() as u64;
        let threshold = multisig.threshold;
        require!(approval_count >= threshold, ErrorCode::InsufficientSigners);
        Ok(())
    }
    pub fn check_if_already_executed(&self) -> Result<(), ErrorCode> {
        require!(!self.did_execute, ErrorCode::TransactionAlreadyExecuted);
        Ok(())
    }
    pub format_ix(&self, multisig_signer) -> Result<Instruction, _> {
        //first build the ix 
        let mut ix = (*self.deref()).to_instruction();
        ix.accounts = ix
            .accounts
            .iter()
            .map(|acc| {
                let mut acc = acc.clone();
                if acc.pubkey == self.multisig_signer.key {
                    acc.is_signer = true;
                }
                acc
            }).collect();
        ix
    }
    pub fn did_execute(&mut self) -> Result<(), ErrorCode> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted);
        }
        self.did_execute = true;
        Ok(());
    }

    pub fn edit_tx_details(
        &mut self,
        program_id: &Pubkey,
        accounts: Vec<TransactionAccount>,
        data: Vec<u8>,
        proposer: Pubkey,
    ) -> Result<(), ErrorCode> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted);
        }
        if accounts.is_empty() || data.is_empty() {
            return Err(ErrorCode::InvalidTransactionDetails);
        }

        if proposer == self.owner {
        self.program_id = *program_id;
        self.accounts = accounts;
        self.data = data;
        }else {
            return Err(ErrorCode::InvalidOwner);

        }
        Ok(())
    }

    pub fn cancel(&mut self, proposer:Pubkey) -> Result<(), ErrorCode> {
        if self.did_execute {
            return Err(ErrorCode::TransactionAlreadyExecuted);
        }
        assert!(!self.owner == proposer, ErrorCode::InvalidOwner);
        Ok(())
    }

    pub fn revokeApproval(&mut self, signer: Pubkey) -> Result<(), ErrorCode> {
        let owner_index = self
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
impl<'info> ToIx for Account<'info, Transaction> {
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


