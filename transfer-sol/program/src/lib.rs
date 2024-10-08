use borsh::{BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    system_instruction,
    system_program,
};

#[derive(BorshDeserialize)]
pub struct TransferInstruction {
    pub value: u64,
}

entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let from = next_account_info(accounts_iter)?;
    let to = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;

    assert!(from.is_writable);
    assert!(from.is_signer);
    assert!(to.is_writable);
    assert!(system_program::check_id(program.key));

    let transfer_instr_data = TransferInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let amount = transfer_instr_data.value;
    msg!("Trying to send amount of {}", amount);

    // Creating the Transfer Instruction, which will be
    // used for invoking the system::transfer program
    let transfer_instr = system_instruction::transfer(&from.key, &to.key, amount);
    invoke(&transfer_instr, &[from.clone(), to.clone()])?;

    msg!("Successfully sent {} lamports from {} to {}", amount, from.key, to.key);

    Ok(())
}
