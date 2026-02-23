use anchor_lang::prelude::*;

declare_id!("BonS3fBk9ufJCduoZoDsFcHs913rt2WhYmdKWbu7cgPp");

#[program]
pub mod zinc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let account_data = &mut ctx.accounts.pda_account;
        // store the address of the user
        account_data.user = *ctx.accounts.user.key;
        // store the canonical bumpd
        account_data.bump = ctx.bumps.pda_account;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        //define the seeds to derive the pda
        seeds = [b"data", user.key().as_ref()],
        // use the canonical bump,
        bump,
        payer = user,
        space = 8 + DataAccount::INIT_SPACE 
    )]

    pub pda_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,
}