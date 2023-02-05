use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::Processor;


//all accounts (read or written) are passed into the entrypoint function
entrypoint!(process_instruction);
fn process_instruction(

    //program_id is the public key of the program
    program_id: &Pubkey,

    //stores state as Solana programs themselves are stateless.
    //programs are accounts that are marked executable
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}