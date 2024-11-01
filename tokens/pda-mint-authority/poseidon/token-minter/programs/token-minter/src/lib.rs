use anchor_lang::prelude::*;

declare_id!("D6HXvkuQVEF1Waki5P3RRtyBkC9Y7xsCVnmBVfi6kvBV");

#[program]
pub mod token_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
