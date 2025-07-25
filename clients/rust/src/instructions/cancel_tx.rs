//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
#[derive(Debug)]
pub struct CancelTx {
      
              
          pub multisig: solana_pubkey::Pubkey,
          
              
          pub transaction: solana_pubkey::Pubkey,
          
              
          pub proposer: solana_pubkey::Pubkey,
      }

impl CancelTx {
  pub fn instruction(&self) -> solana_instruction::Instruction {
    self.instruction_with_remaining_accounts(&[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, remaining_accounts: &[solana_instruction::AccountMeta]) -> solana_instruction::Instruction {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_instruction::AccountMeta::new_readonly(
            self.multisig,
            false
          ));
                                          accounts.push(solana_instruction::AccountMeta::new(
            self.transaction,
            false
          ));
                                          accounts.push(solana_instruction::AccountMeta::new(
            self.proposer,
            true
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let data = borsh::to_vec(&CancelTxInstructionData::new()).unwrap();
    
    solana_instruction::Instruction {
      program_id: crate::SOLANA_CORE_MULTISIG_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct CancelTxInstructionData {
            discriminator: [u8; 8],
      }

impl CancelTxInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [235, 85, 151, 12, 219, 197, 50, 18],
                  }
  }
}

impl Default for CancelTxInstructionData {
  fn default() -> Self {
    Self::new()
  }
}



/// Instruction builder for `CancelTx`.
///
/// ### Accounts:
///
          ///   0. `[]` multisig
                ///   1. `[writable]` transaction
                      ///   2. `[writable, signer]` proposer
#[derive(Clone, Debug, Default)]
pub struct CancelTxBuilder {
            multisig: Option<solana_pubkey::Pubkey>,
                transaction: Option<solana_pubkey::Pubkey>,
                proposer: Option<solana_pubkey::Pubkey>,
                __remaining_accounts: Vec<solana_instruction::AccountMeta>,
}

impl CancelTxBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn multisig(&mut self, multisig: solana_pubkey::Pubkey) -> &mut Self {
                        self.multisig = Some(multisig);
                    self
    }
            #[inline(always)]
    pub fn transaction(&mut self, transaction: solana_pubkey::Pubkey) -> &mut Self {
                        self.transaction = Some(transaction);
                    self
    }
            #[inline(always)]
    pub fn proposer(&mut self, proposer: solana_pubkey::Pubkey) -> &mut Self {
                        self.proposer = Some(proposer);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_instruction::Instruction {
    let accounts = CancelTx {
                              multisig: self.multisig.expect("multisig is not set"),
                                        transaction: self.transaction.expect("transaction is not set"),
                                        proposer: self.proposer.expect("proposer is not set"),
                      };
    
    accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
  }
}

  /// `cancel_tx` CPI accounts.
  pub struct CancelTxCpiAccounts<'a, 'b> {
          
                    
              pub multisig: &'b solana_account_info::AccountInfo<'a>,
                
                    
              pub transaction: &'b solana_account_info::AccountInfo<'a>,
                
                    
              pub proposer: &'b solana_account_info::AccountInfo<'a>,
            }

/// `cancel_tx` CPI instruction.
pub struct CancelTxCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_account_info::AccountInfo<'a>,
      
              
          pub multisig: &'b solana_account_info::AccountInfo<'a>,
          
              
          pub transaction: &'b solana_account_info::AccountInfo<'a>,
          
              
          pub proposer: &'b solana_account_info::AccountInfo<'a>,
        }

impl<'a, 'b> CancelTxCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_account_info::AccountInfo<'a>,
          accounts: CancelTxCpiAccounts<'a, 'b>,
          ) -> Self {
    Self {
      __program: program,
              multisig: accounts.multisig,
              transaction: accounts.transaction,
              proposer: accounts.proposer,
                }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)]) -> solana_program_entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program_entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program_entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_instruction::AccountMeta::new_readonly(
            *self.multisig.key,
            false
          ));
                                          accounts.push(solana_instruction::AccountMeta::new(
            *self.transaction.key,
            false
          ));
                                          accounts.push(solana_instruction::AccountMeta::new(
            *self.proposer.key,
            true
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let data = borsh::to_vec(&CancelTxInstructionData::new()).unwrap();
    
    let instruction = solana_instruction::Instruction {
      program_id: crate::SOLANA_CORE_MULTISIG_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.multisig.clone());
                        account_infos.push(self.transaction.clone());
                        account_infos.push(self.proposer.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_cpi::invoke(&instruction, &account_infos)
    } else {
      solana_cpi::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `CancelTx` via CPI.
///
/// ### Accounts:
///
          ///   0. `[]` multisig
                ///   1. `[writable]` transaction
                      ///   2. `[writable, signer]` proposer
#[derive(Clone, Debug)]
pub struct CancelTxCpiBuilder<'a, 'b> {
  instruction: Box<CancelTxCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CancelTxCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(CancelTxCpiBuilderInstruction {
      __program: program,
              multisig: None,
              transaction: None,
              proposer: None,
                                __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn multisig(&mut self, multisig: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.multisig = Some(multisig);
                    self
    }
      #[inline(always)]
    pub fn transaction(&mut self, transaction: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.transaction = Some(transaction);
                    self
    }
      #[inline(always)]
    pub fn proposer(&mut self, proposer: &'b solana_account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.proposer = Some(proposer);
                    self
    }
            /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program_entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program_entrypoint::ProgramResult {
        let instruction = CancelTxCpi {
        __program: self.instruction.__program,
                  
          multisig: self.instruction.multisig.expect("multisig is not set"),
                  
          transaction: self.instruction.transaction.expect("transaction is not set"),
                  
          proposer: self.instruction.proposer.expect("proposer is not set"),
                    };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct CancelTxCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_account_info::AccountInfo<'a>,
            multisig: Option<&'b solana_account_info::AccountInfo<'a>>,
                transaction: Option<&'b solana_account_info::AccountInfo<'a>>,
                proposer: Option<&'b solana_account_info::AccountInfo<'a>>,
                /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_account_info::AccountInfo<'a>, bool, bool)>,
}

