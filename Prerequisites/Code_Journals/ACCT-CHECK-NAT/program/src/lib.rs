use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};


entrypoint!(process_instruction);

///TC - the process_instruction function is completed below then the entrypoint macro is used to which was imported from the solana_program library "use solana_program"
fn process_instruction(
    ///TC - account data is passed as parameters
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact 
    //      the program ID of your program.

    ///TC - uses native library to check the program ID matches if not, returns an default error from the native library
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };
    
    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you 
    //      try to reach past the end of the iter.

    ///TC - checks to make sure all of the account data is passed by using the .len() method and errors if it is less than 4
    /// as shown in the msg! macro.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.

    ///TC - next_account_info is a native iterator that returns an option and helps assign the correct info into the vector at a specific order
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    
    ///TC - checks the account lamports and verifies that it is not more than zero. Which would otherwise indicate that the account is already initialized. Returns a native error.
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    ///TC - verifies that the account has been created by checking lamports and if = to 0 then throws an error
    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.
    ///TC - verifies the account owner and program_id are the same.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    ///TC - verifies that the program_ID is unique and not of default solana
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}