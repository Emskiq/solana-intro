use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    system_program,
}
use anchor_spl::{
    associated_token,
    token,
}
use mpl_token_metadata::{
    ID as TOKEN_METADATA_ID,
    instruction as token_instruction,
}

declare_id!("AnPHjVj49QAyYeV2KL8vmA7q3FN4PRJzb1hgxYY3yTyt");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
