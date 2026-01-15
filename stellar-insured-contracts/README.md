# Stellar Insured - Soroban Smart Contracts

Complete insurance management system on Stellar Soroban with decentralized governance and risk pooling.

## Architecture

### 1. Policy Contract
Manages insurance policy issuance, renewal, and lifecycle.
- **Issue Policy**: Create new insurance policies with coverage amounts and premiums
- **Renew Policy**: Extend policy duration before expiry
- **Cancel Policy**: Policyholder can cancel active policies
- **Expire Policy**: Mark policies as expired

**Key Functions**:
- `initialize(admin, risk_pool)` - Initialize contract
- `issue_policy(holder, coverage_amount, premium_amount, duration_days, policy_type)` - Issue new policy
- `get_policy(policy_id)` - Retrieve policy details
- `renew_policy(policy_id, duration_days)` - Renew existing policy
- `cancel_policy(policy_id)` - Cancel policy
- `expire_policy(policy_id)` - Mark as expired
- `get_stats()` - Get contract statistics

### 2. Claims Contract
Processes insurance claims with multi-step approval workflow.
- **Submit Claim**: Policyholders submit claims with evidence
- **Approve Claim**: Admin approves valid claims
- **Reject Claim**: Admin rejects invalid claims
- **Settle Claim**: Release funds to claimant

**Key Functions**:
- `initialize(admin, policy_contract, risk_pool)` - Initialize contract
- `submit_claim(policy_id, amount, description, evidence)` - Submit new claim
- `get_claim(claim_id)` - Retrieve claim details
- `approve_claim(claim_id)` - Admin approves claim
- `reject_claim(claim_id)` - Admin rejects claim
- `settle_claim(claim_id)` - Settle approved claim
- `get_stats()` - Get claims statistics

### 3. Risk Pool Contract
Manages liquidity pool for claims settlement.
- **Deposit Liquidity**: Providers deposit XLM to earn rewards
- **Withdraw Liquidity**: Withdraw staked amounts
- **Reserve Liquidity**: Lock funds for pending claims
- **Release Liquidity**: Return reserved funds after settlement

**Key Functions**:
- `initialize(admin, xlm_token, min_provider_stake)` - Initialize pool
- `deposit_liquidity(provider, amount)` - Deposit into pool
- `withdraw_liquidity(provider, amount)` - Withdraw from pool
- `get_pool_stats()` - Pool statistics
- `get_provider_info(provider)` - Provider stake info
- `reserve_liquidity(amount)` - Reserve for claims
- `release_liquidity(amount)` - Release reserved amount

### 4. Governance Contract
Decentralized governance for protocol upgrades and decisions.
- **Create Proposal**: Community members create proposals
- **Vote on Proposal**: Token holders vote on proposals
- **Finalize Proposal**: Execute proposal after voting period

**Key Functions**:
- `initialize(admin, token_contract, voting_period_days, min_voting_percentage)` - Initialize
- `create_proposal(title, description, threshold_percentage)` - Create proposal
- `get_proposal(proposal_id)` - Get proposal details
- `vote(proposal_id, vote_weight, is_yes)` - Vote on proposal
- `finalize_proposal(proposal_id)` - Finalize after voting
- `get_stats()` - Get governance statistics

## Data Structures

### Policy
```
id: u64
holder: Address
coverage_amount: i128
premium_amount: i128
expiry_time: u64
status: PolicyStatus (Active, Expired, Cancelled, Claimed)
policy_type: String
created_at: u64
```

### Claim
```
id: u64
policy_id: u64
claimant: Address
claim_amount: i128
status: ClaimStatus (Pending, Approved, Rejected, Settled)
description: String
created_at: u64
settled_at: u64
evidence: Vec<String>
```

### LiquidityProvider
```
address: Address
stake_amount: i128
contribution_time: u64
share_percentage: i128
```

### Proposal
```
id: u64
title: String
description: String
proposer: Address
yes_votes: i128
no_votes: i128
status: ProposalStatus
created_at: u64
end_time: u64
threshold_percentage: i128
```

## Deployment

1. Build all contracts:
```bash
cd contracts/policy && cargo build --release
cd contracts/claims && cargo build --release
cd contracts/risk_pool && cargo build --release
cd contracts/governance && cargo build --release
```

2. Deploy to Stellar network using Soroban CLI

3. Initialize each contract with proper parameters

## Security Considerations

- **Authorization**: All sensitive operations require authentication
- **State Validation**: Comprehensive checks on contract state transitions
- **Error Handling**: Descriptive error codes for debugging
- **Event Logging**: All important actions emit events
- **Rate Limiting**: Consider implementing rate limits for production

## Token Configuration

Uses Stellar's native XLM token for:
- Premium payments
- Claims settlement
- Liquidity pool deposits
- Governance voting weight

## Testing

Each contract includes:
- State management tests
- Authorization tests
- State transition tests
- Integration tests with other contracts

## Future Enhancements

- Automated claim settlement using oracles
- Parametric insurance products
- Reinsurance integration
- Advanced risk assessment models
- Multi-signature governance
- Cross-chain interoperability
