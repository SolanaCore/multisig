use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Threshold must be greater than zero and less than or equal to number of owners")]
    InvalidThreshold,
    #[msg("The owner is not part of the multisig group")]
    InvalidOwner,
    #[msg("Not enough owners signed this transaction.")]
    InsufficientSigners,
    #[msg("Transaction is already executed")]
    TransactionAlreadyExecuted,
    #[msg("Transaction details provided are invalid")]
    InvalidTransactionDetails
}
