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
    proposal.bump = *ctx.bumps.get("proposal_analysis").unwrap();

    metrics.total_votes = 0;
    metrics.late_votes = 0;
    metrics.late_vote_ratio_bps = 0;
    metrics.bump = *ctx.bumps.get("participation_metrics").unwrap();

    risk.risk_level = 0; // LOW
    risk.computed_at = Clock::get()?.unix_timestamp;
    risk.bump = *ctx.bumps.get("risk_signal").unwrap();

    Ok(())
}