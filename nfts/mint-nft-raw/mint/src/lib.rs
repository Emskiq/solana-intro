use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
    native_token::LAMPORTS_PER_SOL,
};
use spl_token::instruction as token_instruction;
use spl_associated_token_account::instruction as token_account_instruction;

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // What will be passed from our client side
    let mint = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?; // our PDA? (data account)

    // Some initial checks of the accounts passed
    assert!(mint_authority.is_writable);
    assert!(mint_authority.is_signer);
    assert!(mint.is_writable);

    msg!("Creating mint account (just the account, not yet specified for tokens)");
    msg!("Mint: {}", mint.key);
    invoke(
        &system_instruction::create_account(
            &mint_authority.key,
            &mint.key,
            LAMPORTS_PER_SOL,
            82, // apparatnyl this magic number is just like that
            &token_program.key,
        ),
        &[
            mint.clone(),
            mint_authority.clone(),
            token_program.clone(),
        ]
    )?;


    msg!("Initialize of mint account (account holding the NFT/Token info) (the Token)");
    invoke(
        &token_instruction::initialize_mint(
            &token_program.key,
            &mint.key,
            &mint_authority.key,
            Some(&mint_authority.key),
            0
        ).unwrap(),
        &[
            mint.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone()
        ],
    )?;

    msg!("Creating token account for the user/wallet");
    invoke(
        &token_account_instruction::create_associated_token_account(
            &mint_authority.key,
            &mint_authority.key,
            &mint.key,
            &token_program.key
        ),
        &[
            mint.clone(),
            token_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            associated_token_program.clone()
        ]
    )?;

    msg!("Minting tokens to the user's token account");
    msg!("Mint: {}", mint.key);   
    msg!("Token Address: {}", token_account.key);

    invoke(
        &token_instruction::mint_to(
            &token_program.key,
            &mint.key,
            &token_account.key,
            &mint_authority.key,
            &[&mint_authority.key],
            1,
        ).unwrap(),
        &[
            mint.clone(),
            mint_authority.clone(),
            token_account.clone(),
            token_program.clone(),
            rent.clone()
        ]
    )?;

    msg!("Token/NFT minted and granted to the user's token account");


    Ok(())
}
