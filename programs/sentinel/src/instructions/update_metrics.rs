pub fn update_metrics(
    ctx: Context<UpdateMetrics>,
    total_votes: u64,
    late_votes: u64,
) -> Result<()> {

    let metrics = &mut ctx.accounts.participation_metrics;
    let risk = &mut ctx.accounts.risk_signal;

    metrics.total_votes = total_votes;
    metrics.late_votes = late_votes;

    if total_votes > 0 {
        metrics.late_vote_ratio_bps =
            ((late_votes * 10_000) / total_votes) as u16;
    } else {
        metrics.late_vote_ratio_bps = 0;
    }

    let ratio = metrics.late_vote_ratio_bps;

    risk.risk_level = if ratio > 6000 {
        2 
    } else if ratio > 3000 {
        1 
    } else {
        0 
    };

    risk.computed_at = Clock::get()?.unix_timestamp;

    Ok(())
}