use anchor_lang::prelude::*;

#[account]
pub struct ProposalAnalysis {
    pub proposal_pubkey: Pubkey,
    pub voting_start_ts: i64,
    pub voting_end_ts: i64,
    pub bump: u8,
}

#[account]
pub struct ParticipationMetrics {
    pub total_votes: u64,
    pub late_votes: u64,
    pub late_vote_ratio_bps: u16,
    pub bump: u8,
}

#[account]
pub struct RiskSignal {
    pub risk_level: u8,      
    pub computed_at: i64,
    pub bump: u8,
}

impl ProposalAnalysis {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1;
}

impl ParticipationMetrics {
    pub const LEN: usize = 8 + 8 + 8 + 2 + 1;
}

impl RiskSignal {
    pub const LEN: usize = 8 + 1 + 8 + 1;
}