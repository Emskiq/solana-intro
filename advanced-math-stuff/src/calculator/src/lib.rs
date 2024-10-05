use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::calculator::CalculatorInstructions;

mod calculator;



#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Calculator {
    pub value: u32,
}


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Always checking whether the current account calling
    // the insturction is the actual program account.
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?; // This is the way for getting the next account

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut calc = Calculator::try_from_slice(&account.data.borrow())?;

    let calc_instruction = CalculatorInstructions::try_from_slice(instruction_data)?;

    calc.value = calc_instruction.evaluate(calc.value);

    calc.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("After running the instruction Value is: {}", calc.value);

    Ok(())
}
