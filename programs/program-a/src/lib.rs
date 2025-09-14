use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("7ptinUrw6pRPZ8Rky9nPfjyF13RTnL68HHS6MX1omHUD");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let pda_account = &ctx.accounts.pda_account;
        let signer = &ctx.accounts.signer;
        let signer_address = signer.key();
        let instruction = &system_instruction::transfer(&pda_account.key(), &signer.key(), 100);
        let bump = ctx.bumps.pda_account;

        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        let signers_seeds: &[&[&[u8]]] = &[&[b"pda", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, &account_infos, signers_seeds)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        seeds = [b"pda", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
//    pub program_b: Program<'info, ProgramB>,
}
