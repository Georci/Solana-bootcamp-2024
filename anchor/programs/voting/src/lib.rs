#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting {
    use anchor_lang::context;

    use super::*;

    // 第一个指令
    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = _poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;

        Ok(())
    }

    // 第二个指令
    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        candidate_name: String,
        _poll_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_name = candidate_name;
        candidate.candidate_vote = 0;

        Ok(())
    }

    // 第三个指令
    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let vote_account = &mut ctx.accounts.poll_account;
        let candidate_account = &mut ctx.accounts.candidate_account;

        let current_time = Clock::get()?.unix_timestamp;
        if current_time > (vote_account.poll_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (vote_account.poll_start as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        msg!("wuxizhi1haoshuai1");
        candidate_account.candidate_vote += 1;
        msg!("candidateVote is :{}", candidate_account.candidate_vote);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, Candidate>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + Candidate::INIT_SPACE, 
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()], 
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(20)]
    pub candidate_name: String,
    pub candidate_vote: u64,
}



#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}


