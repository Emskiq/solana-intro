use anchor_lang::prelude::*;

pub mod mint;
pub mod sell;

use mint::*;
use sell::*;

declare_id!("7vS3Pyu8NJVqdZ61De9zTSGm288c8Ac2k7SQFuAHtPMi");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        mint::mint(ctx, metadata_title, metadata_symbol, metadata_uri)
    }

    pub fn sell(
        ctx: Context<SellNFT>,
        sell_amount: u64,
    ) -> Result<()> {
        sell::sell(ctx, sell_amount)
    }
}
