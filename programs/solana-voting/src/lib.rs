use anchor_lang::prelude::*;
use vote_count::cpi::accounts::UpdateVoteCount;
use vote_count::program::VoteCount;
use vote_count::{ Count };
use vote_record::cpi::accounts::UpdateRecord;
use vote_record::program::VoteRecord;
use vote_record::{ Record };

declare_id!("3yFnjbmi9Fhd999rLMM5hiFen2f1u4LLqTedASi66jx9");

#[program]
pub mod solana_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> ProgramResult {
        ctx.accounts.proposal_account.bump = bump;
        ctx.accounts.proposal_account.id = 0;
        Ok(())
    }

    pub fn add_proposal(ctx: Context<AddProposal>, description: String) -> ProgramResult {
        ctx.accounts.proposal_account.id += 1;
        ctx.accounts.proposal_account.description = description;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, agree: bool) -> ProgramResult {
        if ctx.accounts.record_account.voted_proposals.contains(&ctx.accounts.proposal_account.key()) {
            Err(ErrorCode::ProposalAlreadyVoted.into())
        }
        else {
            let count_program = ctx.accounts.count_program.to_account_info();
            let count_accounts = UpdateVoteCount {
                count_account: ctx.accounts.count_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info()
            };
            let count_ctx = CpiContext::new(count_program, count_accounts);
            vote_count::cpi::update_vote_count(count_ctx, agree)?;

            let record_program = ctx.accounts.record_program.to_account_info();
            let record_accounts = UpdateRecord {
                record_account: ctx.accounts.record_account.to_account_info(),
                user: ctx.accounts.user.to_account_info()
            };
            let record_ctx = CpiContext::new(record_program, record_accounts);
            vote_record::cpi::update_record(record_ctx, ctx.accounts.proposal_account.key(), 10)?;

            Ok(())
        }
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"proposal".as_ref()],
        bump = bump,
        payer = authority,
        space = 500
    )]
    pub proposal_account: Account<'info, Proposal>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddProposal<'info> {
    #[account(mut)]
    pub proposal_account: Account<'info, Proposal>
}

#[derive(Accounts)]
pub struct Vote<'info> {
    pub proposal_account: Account<'info, Proposal>,
    pub count_program: Program<'info, VoteCount>,
    #[account(mut, has_one = authority)]
    pub count_account: Account<'info, Count>,
    pub authority: Signer<'info>,
    pub record_program: Program<'info, VoteRecord>,
    #[account(mut, has_one = user)]
    pub record_account: Account<'info, Record>,
    pub user: Signer<'info>
}

#[account]
pub struct Proposal {
    pub bump: u8,
    pub id: u8,
    pub description: String
}

#[error]
pub enum ErrorCode {
    #[msg("Proposal has already been voted")]
    ProposalAlreadyVoted
}
