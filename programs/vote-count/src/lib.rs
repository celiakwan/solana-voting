use anchor_lang::prelude::*;

declare_id!("Ga9XxaFMFg2wFfj6fgHreuLaJw5kBPhNjoBqSJCSZyCN");

#[program]
pub mod vote_count {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8, proposal: Pubkey) -> ProgramResult {
        ctx.accounts.count_account.bump = bump;
        ctx.accounts.count_account.authority = *ctx.accounts.authority.key;
        ctx.accounts.count_account.proposal = proposal;
        Ok(())
    }

    pub fn update_vote_count(ctx: Context<UpdateVoteCount>, agree: bool) -> ProgramResult {
        if agree {
            ctx.accounts.count_account.agree += 1;
        }
        else {
            ctx.accounts.count_account.disagree += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, proposal: Pubkey)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"count".as_ref(), proposal.as_ref()],
        bump = bump,
        payer = authority
    )]
    pub count_account: Account<'info, Count>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateVoteCount<'info> {
    #[account(mut, has_one = authority)]
    pub count_account: Account<'info, Count>,
    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct Count {
    pub bump: u8,
    pub authority: Pubkey,
    pub proposal: Pubkey,
    pub agree: u64,
    pub disagree: u64
}
