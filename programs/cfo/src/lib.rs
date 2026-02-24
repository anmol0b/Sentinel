use anchor_lang::prelude::*;

declare_id!("B2zKeoeD1XQu7JqnpcPDiJXowsA9MEhDvZWkYBum5Rcj");

#[program]
pub mod cfo {
    use super::*;

    pub fn register_proposal(
        ctx: Context<RegisterProposal>,
        proposal_pubkey: Pubkey,
        voting_start_ts: i64,
        voting_end_ts: i64,
    ) -> Result<()> {

        let proposal = &mut ctx.accounts.proposal_analysis;
        let metrics = &mut ctx.accounts.participation_metrics;
        let risk = &mut ctx.accounts.risk_signal;


        proposal.proposal_pubkey = proposal_pubkey;
        proposal.voting_start_ts = voting_start_ts;
        proposal.voting_end_ts = voting_end_ts;
        proposal.bump = ctx.bumps.proposal_analysis;

        metrics.total_votes = 0;
        metrics.late_votes = 0;
        metrics.late_vote_ratio_bps = 0;
        metrics.bump = ctx.bumps.participation_metrics;

        risk.risk_level = 0;
        risk.computed_at = Clock::get()?.unix_timestamp;
        risk.bump = ctx.bumps.risk_signal;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(proposal_pubkey: Pubkey)]
pub struct RegisterProposal<'info> {

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ProposalAnalysis::LEN,
        seeds = [b"proposal", proposal_pubkey.as_ref()],
        bump
    )]
    pub proposal_analysis: Account<'info, ProposalAnalysis>,

    #[account(
        init,
        payer = signer,
        space = ParticipationMetrics::LEN,
        seeds = [b"metrics", proposal_pubkey.as_ref()],
        bump
    )]
    pub participation_metrics: Account<'info, ParticipationMetrics>,

    #[account(
        init,
        payer = signer,
        space = RiskSignal::LEN,
        seeds = [b"risk", proposal_pubkey.as_ref()],
        bump
    )]
    pub risk_signal: Account<'info, RiskSignal>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProposalAnalysis {
    pub proposal_pubkey: Pubkey,
    pub voting_start_ts: i64,
    pub voting_end_ts: i64,
    pub bump: u8,
}

impl ProposalAnalysis {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1;
}

#[account]
pub struct ParticipationMetrics {
    pub total_votes: u64,
    pub late_votes: u64,
    pub late_vote_ratio_bps: u16,
    pub bump: u8,
}

impl ParticipationMetrics {
    pub const LEN: usize = 8 + 8 + 8 + 2 + 1;
}

#[account]
pub struct RiskSignal {
    pub risk_level: u8,     // 0=LOW,1=MEDIUM,2=HIGH
    pub computed_at: i64,
    pub bump: u8,
}

impl RiskSignal {
    pub const LEN: usize = 8 + 1 + 8 + 1;
}