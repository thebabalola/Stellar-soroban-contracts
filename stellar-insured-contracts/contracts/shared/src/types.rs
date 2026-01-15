use soroban_sdk::{contracttype, Address, String};

#[derive(Clone, Debug)]
#[contracttype]
pub struct InsuranceProduct {
    pub name: String,
    pub description: String,
    pub min_coverage: i128,
    pub max_coverage: i128,
    pub base_premium_rate: i128, // basis points
    pub coverage_period_days: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct ClaimEvidence {
    pub evidence_type: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct AuditLog {
    pub contract_name: String,
    pub action: String,
    pub actor: Address,
    pub timestamp: u64,
    pub details: String,
}
