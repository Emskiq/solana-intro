use anchor_lang::prelude::*;

declare_id!("13rvrHn5Lw7fZLYJdc23RtB4RXcQsPT9tnjgXXsaiArX");

#[program]
pub mod pda {
    use super::*;

    pub fn create_ledger(
        ctx: Context<CreateLedger>,
        color: String,
    ) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger_account;
        ledger.color = color;
        ledger.balance = 0;

        Ok(())
    }

    pub fn modify_ledger(
        ctx: Context<ModifyLedger>,
        new_balance: u32,
    ) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger_account;
        ledger.balance = new_balance;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(color: String)]
pub struct CreateLedger<'info> {
    #[account(
        init, // This is where the magic happens -> creating by anchor
        payer = wallet,
        space = 82,
        seeds = [wallet.key().as_ref(), b"_", color.as_ref()],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLedger<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[account]
pub struct Ledger {
    pub color: String,
    pub balance: u32,
}
