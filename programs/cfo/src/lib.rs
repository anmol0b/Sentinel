use anchor_lang::prelude::*;

declare_id!("B2zKeoeD1XQu7JqnpcPDiJXowsA9MEhDvZWkYBum5Rcj");

#[program]
pub mod cfo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
