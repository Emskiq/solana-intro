use anchor_lang::prelude::*;

declare_id!("13rvrHn5Lw7fZLYJdc23RtB4RXcQsPT9tnjgXXsaiArX");

#[program]
pub mod pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
