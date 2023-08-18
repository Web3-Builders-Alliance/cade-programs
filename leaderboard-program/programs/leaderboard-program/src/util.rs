use anchor_lang::prelude::*;
use anchor_lang::system_program;
use std::str::FromStr;

pub fn to_pubkey(string: &str) -> Pubkey {
    Pubkey::from_str(&string).expect("Error parsing public key from string.")
}

pub fn set_and_maybe_realloc<'info, T>(
    account: &mut Account<'info, T>,
    new_data: T,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()>
where
    T: AccountDeserialize
        + AccountSerialize
        + borsh::BorshDeserialize
        + borsh::BorshSerialize
        + Clone + anchor_lang::Owner,
{
    let account_info = account.to_account_info();

    let new_account_size = (new_data.try_to_vec()?).len();
    if new_account_size > account_info.data_len() {
        let lamports_required = (Rent::get()?).minimum_balance(new_account_size);
        let additional_rent_to_fund = lamports_required - account_info.lamports();

        system_program::transfer(
            CpiContext::new(
                system_program,
                system_program::Transfer {
                    from: payer,
                    to: account_info.clone(),
                },
            ),
            additional_rent_to_fund,
        )?;

        account_info.realloc(new_account_size, false)?;
    }
    account.set_inner(new_data);
    Ok(())
}