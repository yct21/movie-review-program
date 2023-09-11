use crate::error::Error;
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize)]
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
    UpdateMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
}

impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        MovieInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::from(Error::ParseMovieReviewPayloadFailed))
    }
}
