use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instructions as token_instruction,
    },
};

use anchor_spl::token::Token;
use anchor_spl::associated_token::AssociatedToken;

declare_id!("FbNTR9iqZqxsUAdmUV4nZPigj6htoy6EZ3WYeCKg3U11");

#[program]
pub mod mint_nft {
    use super::*;
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

    use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;
    use mpl_token_metadata::types::DataV2;

    use anchor_spl::metadata;

    pub fn mint(
        ctx: Context<MintNft>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {

        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            LAMPORTS_PER_SOL,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());    
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ),
        )?;

        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());   
        msg!("Token Address: {}", &ctx.accounts.token_account.key());     
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());

        // This will be our instruction
        let create_metadata_account_v3_instructions =
            token_instruction::CreateMetadataAccountV3{
                metadata: ctx.accounts.metadata.key(),
                mint: ctx.accounts.mint.key(),
                mint_authority: ctx.accounts.mint_authority.key(),
                payer: ctx.accounts.mint_authority.key(),
                update_authority: (ctx.accounts.mint_authority.key(), false),
                system_program: ctx.accounts.system_program.key(),
                rent: None,
            }.instruction(CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                name: metadata_title,
                symbol: metadata_symbol,
                uri: metadata_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: false,
            collection_details: None,
        });

        invoke(
            &create_metadata_account_v3_instructions,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Creating master edition metadata account...");
        msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
        let create_master_edition_instruction =
            token_instruction::CreateMasterEditionV3{
                edition: ctx.accounts.master_edition.key(),
                mint: ctx.accounts.mint.key(),
                update_authority: ctx.accounts.mint_authority.key(),
                mint_authority: ctx.accounts.mint_authority.key(),
                payer: ctx.accounts.mint_authority.key(),
                system_program: ctx.accounts.system_program.key(),
                token_program: ctx.accounts.token_program.key(),
                rent: Some(ctx.accounts.rent.key()),
                metadata: ctx.accounts.metadata.key(),
            }.instruction(token_instruction::CreateMasterEditionV3InstructionArgs { max_supply: Some(0) });

        // invoke(
        //     &create_master_edition_instruction,
        //     &[
        //         ctx.accounts.master_edition.to_account_info(),
        //         ctx.accounts.metadata.to_account_info(),
        //         ctx.accounts.mint.to_account_info(),
        //         ctx.accounts.token_account.to_account_info(),
        //         ctx.accounts.mint_authority.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //         ctx.accounts.token_program.to_account_info(),
        //         ctx.accounts.rent.to_account_info(),
        //     ],
        // )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}
