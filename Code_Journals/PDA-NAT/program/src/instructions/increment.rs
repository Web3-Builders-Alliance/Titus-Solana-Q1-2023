use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult, 
};

use crate::state::PageVisits;


pub fn increment_page_visits(
    accounts: &[AccountInfo],
) -> ProgramResult {

    ///TC - mutable reference to accounts and makes it iterable
    let accounts_iter = &mut accounts.iter();
    ///TC - iterates through the account data
    let page_visits_account = next_account_info(accounts_iter)?;

    ///TC - takes the data from page_visits_account and creates a str slice of the data. 
    let page_visits = &mut PageVisits::try_from_slice(&page_visits_account.data.borrow())?;
    page_visits.increment();
    page_visits.serialize(&mut &mut page_visits_account.data.borrow_mut()[..])?;
    Ok(())
}