# Sentinel — Governance Risk Oracle for Solana DAOs

Sentinel is an advisory on-chain governance risk oracle designed for Solana DAOs.

It analyzes participation patterns in governance proposals and emits deterministic on-chain risk signals before execution — without interfering with DAO autonomy.

Sentinel acts as a coordination awareness layer, helping DAOs detect governance fragility while preserving decentralization.

---

## 🧠 Why Sentinel?

Most DAO governance systems evaluate proposals only at execution time:

✔ quorum reached  
✔ vote passed  

But they do not evaluate:

❌ rushed participation  
❌ late vote dominance  
❌ weak consensus formation  

This can result in coordination failures — where technically valid proposals lack meaningful legitimacy.

Sentinel introduces a neutral, on-chain advisory layer to detect these patterns early — without enforcing outcomes.

---

## 🏗️ Architecture

### High-Level Flow

Proposal → Participation Tracking → Risk Computation → Advisory Signal

---

### 📊 End-to-End System Overview

![End-to-End Overview](./docs/END%20TO%20END%20Overview%20Diagram.png)

---

### 🔁 Register Proposal — On-Chain Flow

![Register Proposal Flow](./docs/Register%20Proposal%20—%20On-Chain%20Flow.png)

---

### 📉 Update Metrics & Compute Risk

![Update Metrics Flow](./docs/Update%20Metrics%20and%20Compute%20Risk.png)

---

## 🧱 On-Chain Design

Sentinel maintains three deterministic accounts per proposal using Program Derived Addresses (PDAs).

### ProposalAnalysis PDA

Stores proposal metadata.

Seed:

["proposal", proposal_pubkey]

Fields:

- proposal_pubkey  
- voting_start_ts  
- voting_end_ts  

---

### ParticipationMetrics PDA

Tracks participation behavior.

Seed:

["metrics", proposal_pubkey]

Fields:

- total_votes  
- late_votes  
- late_vote_ratio_bps  

---

### RiskSignal PDA

Stores advisory output.

Seed:

["risk", proposal_pubkey]

Fields:

- risk_level  
- computed_at  

## 📊 Risk Model (MVP)

Sentinel uses a deterministic late-vote concentration model.

| Late Vote Ratio | Risk Level |
|-----------------|------------|
| ≤ 30%           | LOW        |
| > 30%           | MEDIUM     |
| > 60%           | HIGH       |

The resulting risk signal is stored on-chain as an advisory output.

Sentinel does not block execution — it only surfaces governance coordination risk.


## 🚀 Devnet Deployment

Program deployed on Solana Devnet.

Program ID:

PASTE_PROGRAM_ID_HERE

Explorer:

https://explorer.solana.com/address/PASTE_PROGRAM_ID_HERE?cluster=devnet