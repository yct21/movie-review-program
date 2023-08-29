use crate::error::Error;
use crate::state::MovieAccountState;
use borsh::BorshSerialize;
use solana_program::account_info::next_account_info;
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::IsInitialized;
use solana_program::system_instruction;
use solana_program::sysvar::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("正在添加电影评论...");
    msg!("标题: {}", title);
    msg!("评分: {}", rating);
    msg!("描述: {}", description);

    // 获取账户迭代器
    let account_info_iter = &mut accounts.iter();

    // 获取账户
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // add check here
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // 构造PDA账户
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes()], program_id);

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if !(1..=5).contains(&rating) {
        msg!("Rating cannot be higher than 5");
        return Err(Error::InvalidRating.into());
    }

    // 计算所需的账户大小
    let total_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(Error::InvalidDataLength.into());
    }

    // 计算所需的租金
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(total_len);

    // 创建账户
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            total_len
                .try_into()
                .map_err(|_| Error::ConvertUsizeToU64Failed)?,
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), title.as_bytes(), &[bump_seed]]],
    )?;

    msg!("创建PDA: {}", pda);

    msg!("解包状态账户");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("借用账户数据");

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("序列化账户");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("状态账户序列化");

    Ok(())
}

pub fn update_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Updating movie review...");
    msg!("Update Movie is : {}", _title);
    msg!("update Movie({}) rating to {}", _title, rating);
    msg!("update Movie({}) description to {}", _title, description);

    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    // Derive PDA and check that it matches client
    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), account_data.title.as_bytes()],
        program_id,
    );

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(Error::InvalidPDA.into());
    }

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(Error::UninitializedAccount.into());
    }

    if !(1..=5).contains(&rating) {
        msg!("Rating cannot be higher than 5");
        return Err(Error::InvalidRating.into());
    }

    let total_len: usize = 1 + 1 + (4 + account_data.title.len()) + (4 + description.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(Error::InvalidDataLength.into());
    }

    account_data.rating = rating;
    account_data.description = description;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}
