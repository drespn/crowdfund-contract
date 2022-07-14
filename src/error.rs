use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum CampaignError {
    ///Invalid Instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<CampaignError> for ProgramError {
    fn from(e: CampaignError) -> Self {
        ProgramError::Custom(e as u32)
    }
}