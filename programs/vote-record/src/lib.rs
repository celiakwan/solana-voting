use anchor_lang::prelude::*;

declare_id!("8kJ6Ny1TiBaLSJ5Tw3sHyWPvH1iP51rUrY4V48N4nrAR");

#[program]
pub mod vote_record {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> ProgramResult {
        ctx.accounts.record_account.bump = bump;
        ctx.accounts.record_account.user = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn update_record(
        ctx: Context<UpdateRecord>,
        voted_proposal: Pubkey,
        rewards: u64,
    ) -> ProgramResult {
        ctx.accounts
            .record_account
            .voted_proposals
            .push(voted_proposal);
        ctx.accounts.record_account.rewards += rewards;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"record".as_ref(), user.key().as_ref()],
        bump = bump,
        payer = user,
        space = 500
    )]
    pub record_account: Account<'info, Record>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRecord<'info> {
    #[account(mut, has_one = user)]
    pub record_account: Account<'info, Record>,
    pub user: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Record {
    pub bump: u8,
    pub user: Pubkey,
    pub voted_proposals: Vec<Pubkey>,
    pub rewards: u64,
}
