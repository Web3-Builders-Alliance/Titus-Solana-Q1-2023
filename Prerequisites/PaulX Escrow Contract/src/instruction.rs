use std::convert::TryInto;
use solana_program::program_error::ProgramError;


use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    },
    Exchange {
        amount: u64
    },
    ResetTimeLock {},
    Cancel {},
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        ///split_first() method is taking the first element of the function param ("input" which is an array slice of u8) and places it into a tuple (tag, rest). "tag" is now a borrowed u8 value and "rest" remains a borrowed u8 array. If the iteration fails, ok_or is used to supply an error message.
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            ///match is used to look at the first byte of "tag" to determine how to decode the input

            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?
            },
            2 => Self::ResetTimeLock {  },
            3 => Self::Cancel {  },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    ///the aim is to take the "rest" and convert to u64 for account data
    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
        ///.get is used to get the 8 bytes of the array = ..8 and return a Some(&[u8]) option value
            .get(..8) 
        ///.and_then is used to take the Some(&[u8]) option value convert into Result (try_into) and .ok converts back to option
            .and_then(|slice| slice.try_into().ok())
        ///takes a LITTLE ENDIAN byte array and converts it to a native endian integer value
            .map(u64::from_le_bytes)
        ///value is "Ok_or" give the error message in ()
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}

// pub fn init_escrow(
//     program_id: &PubKey,
//     initiator: &PubKey,
//     pda_token_account: &PubKey,
//     init_token_account: &PubKey,
//     escrow_account:&PubKey,
//     token_program:&PubKey,
//     amount:u64,
// ) -> Result<Instruction, ProgramError> {

// }