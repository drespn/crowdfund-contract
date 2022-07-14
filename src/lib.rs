use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
use borsh::{BorshSerialize, BorshDeserialize};

pub mod error;
use crate::error::CampaignError::InvalidInstruction;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    //process the passed in params

    //Assuming first byte tells us what we want to do, lets match that
    let(fbyte,rest) =  instruction_data.split_first().ok_or(InvalidInstruction)?;
    //After error handling, need to list out the different scenarios
    //If 0, start a campaign
    match fbyte {
        //start campiagn
        0 => {
            let iterable_accounts = &mut accounts.iter();

        let initializer_acc = next_account_info(iterable_accounts)?;
        //NOTE: First 4 bytes - either account to be created as campaign or funded
        //NOTE: Next 4 bytes  - amount campaign wants
        //NOTE: rest_of_data -  description string
        let amount = rest
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        //getting the description
        //make a utf8 from the rest of the slice to a vector and unwrap it
        let description = String::from_utf8(rest[9..].to_vec()).unwrap();

        //the data from the passed account info
        let mut campaign_account_data = CampaignAccount::try_from_slice(&initializer_acc.data.borrow())?;

            campaign_account_data.campaign_amounts = amount;
            campaign_account_data.campaign_descriptions = description;
            campaign_account_data.campaign_fulfilled = 0;

            campaign_account_data.serialize(&mut &mut initializer_acc.data.borrow_mut()[..])?;

                  
         }
        2 =>{
            let iterable_accounts = &mut accounts.iter();

            let initializer_acc = next_account_info(iterable_accounts)?;

            //after getting the account, get the data
            let campaign_account_data = CampaignAccount::try_from_slice(&initializer_acc.data.borrow())?;

            msg!("{}",campaign_account_data.campaign_amounts-campaign_account_data.campaign_fulfilled);
        }
        _=>{}

    }
    
    Ok(())
}
//create campaign structure
#[derive(BorshDeserialize,BorshSerialize,Debug)]
pub struct CampaignAccount {
    
    pub campaign_owner: Pubkey,
    pub campaign_amounts: u64,
    pub campaign_descriptions: String,
    pub campaign_fulfilled: u64
}

//defining the instructions based on what we want to do
//1. Initialize a campaign
//2. Fund a campaign
//3. Get funds needed to reach goal
//4. Widthdrawal funds and close campaign 




