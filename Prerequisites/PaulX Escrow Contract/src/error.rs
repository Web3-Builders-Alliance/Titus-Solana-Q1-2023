use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// uses 'this error' library to easily create errors via #[error("_")]
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    #[error("Amount Overflow")]
    AmountOverflow
    ///add error for canceling after escrow swap complete?
}

impl From<EscrowError> for ProgramError {
    /// function used to conver the custom enum errors into Solana Program Errors
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}