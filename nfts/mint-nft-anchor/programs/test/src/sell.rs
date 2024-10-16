use {
    anchor_lang::{
        prelude::*, system_program,
    },
    anchor_spl::{
        associated_token::{self, AssociatedToken},
        token::{self, Token, TokenAccount, Mint},
    },
};


pub fn sell(
    ctx: Context<SellNFT>,
    sell_amount: u64,
) -> Result<()> {
    msg!("Initiating transfer of {} lamports...", sell_amount);
    msg!("Purchaser (sending lamports): {}", &ctx.accounts.buyer_authority.key());
    msg!("Seller (receiving lamports): {}", &ctx.accounts.seller_authority.key());
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer_authority.to_account_info(),
                to: ctx.accounts.seller_authority.to_account_info(),
            }
        ),
        sell_amount,
    )?;
    msg!("Lamports transferred successfully.");

    msg!("Creating buyer token account...");
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());
    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ),
    )?;

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.seller_token_account.key());
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.seller_authority.to_account_info(),
            }),
        1
    )?;

    msg!("NFT transferred successfully.");
    msg!("Sale completed successfully!");
    Ok(())
}


#[derive(Accounts)]
pub struct SellNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub seller_authority: Signer<'info>, // or signer/payer

    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>, // or signer/payer

    #[account(mut)]
    pub buyer_authority: Signer<'info>, // or signer/payer

    /// CHECK: We will create it with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,

    // System variable accounts or system programs, that Anchor will automatically deduce"
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
