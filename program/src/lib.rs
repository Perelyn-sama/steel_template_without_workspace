mod add;
pub mod api;
mod initialize;

use add::*;
use api::prelude::*;
use initialize::*;

use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ID, program_id, data)?;

    match ix {
        {name_lowercase}Instruction::Initialize => process_initialize(accounts, data)?,
        {name_lowercase}Instruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
