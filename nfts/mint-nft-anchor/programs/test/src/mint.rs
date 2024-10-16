use {
    anchor_lang::{
        prelude::*,
        system_program,
        solana_program::native_token::LAMPORTS_PER_SOL,
    },
    anchor_spl::{
        associated_token::{self, AssociatedToken},
        token::{self, Token},
    },
    anchor_spl::metadata::{
        create_metadata_accounts_v3,
        CreateMetadataAccountsV3,
        // Uncomment if you find how to not use that much CU
        // create_master_edition_v3,
        // CreateMasterEditionV3,
        Metadata,
        mpl_token_metadata::types::{
            DataV2, Creator
        },
    },
};


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
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.metadata.to_account_info(),
            CreateMetadataAccountsV3{
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                payer: ctx.accounts.mint_authority.to_account_info(),
                mint_authority: ctx.accounts.mint_authority.to_account_info(),
                update_authority: ctx.accounts.mint_authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            }),
            DataV2 {
                name: metadata_title,
                symbol: metadata_symbol,
                uri: metadata_uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![
                    Creator {
                        address: ctx.accounts.mint_authority.key(),
                        verified: true,
                        share: 100, // TODO: See whether that needs to be changed
                    },
                ]),
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
            )?;

    msg!("Creating master edition metadata account...");
    msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
    // XXX: This instruction is causing exceed of compute
    //      units and that's why its commented out.
    // create_master_edition_v3(
    //     CpiContext::new(
    //         ctx.accounts.token_metadata_program.to_account_info(),
    //         CreateMasterEditionV3{
    //             edition: ctx.accounts.master_edition.to_account_info(),
    //             payer: ctx.accounts.mint_authority.to_account_info(),
    //             mint: ctx.accounts.mint.to_account_info(),
    //             metadata: ctx.accounts.metadata.to_account_info(),
    //             mint_authority: ctx.accounts.mint_authority.to_account_info(),
    //             update_authority: ctx.accounts.mint_authority.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //             token_program: ctx.accounts.token_program.to_account_info(),
    //             rent: ctx.accounts.rent.to_account_info(),
    //         }),
    //         Some(1),
    // )?;

    msg!("Minted NFT successfully");

    Ok(())
}


#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>, // or signer/payer

    /// CHECK: We will create it with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata".as_ref(), token_metadata_program.key().as_ref(), mint.key().as_ref(), b"edition".as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK:
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"metadata".as_ref(), token_metadata_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK:
    pub metadata: UncheckedAccount<'info>,

    // System variable accounts or system programs, that Anchor will automatically deduce"
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
}
