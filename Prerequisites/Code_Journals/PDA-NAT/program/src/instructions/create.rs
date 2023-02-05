use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult, 
    program::{ invoke, invoke_signed },
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};

use crate::state::PageVisits;


pub fn create_page_visits(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    page_visits: PageVisits,
) -> ProgramResult {
    ///TC - takes items of AccountInfo array, iterates then assigns data to variables.
    let accounts_iter = &mut accounts.iter();
    let page_visits_account = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;


    ///TC - takes page visits, attempt to serialize into a vector of bytes. a vector then determine the length
    let account_span = (page_visits.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    ///TC - attempt to sign with PDA account data
    invoke_signed(
        
        &system_instruction::create_account(
            &payer.key,
            &page_visits_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), page_visits_account.clone(), system_program.clone()
        ],
        &[&[
            PageVisits::SEED_PREFIX.as_bytes().as_ref(),
            user.key.as_ref(),
            &[page_visits.bump],
        ]]
    )?;

    Ok(())
}

