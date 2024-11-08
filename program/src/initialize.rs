use crate::api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, counter_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    counter_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[COUNTER], &ID)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize counter.
    create_account::<Counter>(counter_info, system_program, signer_info, &ID, &[COUNTER])?;
    let counter = counter_info.as_account_mut::<Counter>(&ID)?;
    counter.value = 0;

    Ok(())
}