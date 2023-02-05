use borsh::{ BorshDeserialize, BorshSerialize };


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IncrementPageVisits {}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PageVisits {
    pub page_visits: u32,
    pub bump: u8,
}

impl PageVisits {
    ///TC - specifies the account space as a usize type
    pub const ACCOUNT_SPACE: usize = 8 + 32;

    ///TC - uses a borrows string slice with a static lifetime as a seed prefix
    pub const SEED_PREFIX: &'static str = "page_visits";


    ///TC - function used to create new public key using page visits and ed2559 bump
    pub fn new(page_visits: u32, bump: u8) -> Self {
        PageVisits {
            page_visits,
            bump,
        }
    }
    ///TC increments page visits by 1
    pub fn increment(&mut self) {
        self.page_visits += 1;
    }
}