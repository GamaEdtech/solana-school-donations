use anchor_lang::prelude::*;

declare_id!("2qSmTUsD7B9dyMBeFLLSpdMdASKZ8LMeaUnZdnKFhM4v");

#[program]
pub mod solana_school_donation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
