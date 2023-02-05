use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions;
use crate::state::PageVisits;
use crate::state::IncrementPageVisits;


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    ///TC - match block uses borrowed instruction_data, deserializes the slice of bytes to allow for the creation of 
    /// page_visits state
    match PageVisits::try_from_slice(&instruction_data) {
        Ok(page_visits) => return instructions::create::create_page_visits(
            program_id, accounts, page_visits
        ),
        Err(_) => {},
    };
    ///TC - match block uses borrowed instruction_data, deserializes then increments the page_visit count.
    match IncrementPageVisits::try_from_slice(&instruction_data) {
        Ok(_) => return instructions::increment::increment_page_visits(
            accounts
        ),
        Err(_) => {},
    };

    ///TC - In all instances, nothing is changed with an error and the 'InvalidInstructionData' message is called.
    Err(ProgramError::InvalidInstructionData)
}