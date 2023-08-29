use crate::instruction::MovieInstruction;
use crate::processor::{add_movie_review, update_movie_review};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Movie Review Program");
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, rating, description),
        // add UpdateMovieReview to match against our new data structure
        MovieInstruction::UpdateMovieReview {
            title,
            rating,
            description,
        } => {
            // make call to update function that we'll define next
            update_movie_review(program_id, accounts, title, rating, description)
        }
    }
}
