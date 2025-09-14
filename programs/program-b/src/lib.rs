use anchor_lang::prelude::*;

declare_id!("EZ5jYYU9wmXcvco1hq12ViTck7oWu9REzAWtX3vfuhQR");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program B");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_account: Signer<'info>,
}
