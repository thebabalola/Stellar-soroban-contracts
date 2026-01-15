#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
    Settled,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Claim {
    pub id: u64,
    pub policy_id: u64,
    pub claimant: Address,
    pub claim_amount: i128,
    pub status: ClaimStatus,
    pub description: String,
    pub created_at: u64,
    pub settled_at: u64,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct ClaimsData {
    pub next_claim_id: u64,
    pub total_claimed: i128,
    pub total_settled: i128,
    pub admin: Address,
    pub policy_contract: Address,
    pub risk_pool: Address,
}

#[contract]
pub struct ClaimsContract;

#[contractimpl]
impl ClaimsContract {
    /// Initialize the claims contract
    pub fn initialize(env: Env, admin: Address, policy_contract: Address, risk_pool: Address) {
        let storage = env.storage().persistent();
        
        admin.require_auth();
        
        let data = ClaimsData {
            next_claim_id: 1,
            total_claimed: 0,
            total_settled: 0,
            admin: admin.clone(),
            policy_contract,
            risk_pool,
        };
        
        storage.set(&symbol_short!("data"), &data);
    }

    /// Submit a new insurance claim
    pub fn submit_claim(
        env: Env,
        policy_id: u64,
        claim_amount: i128,
        description: String,
        evidence: Vec<String>,
    ) -> u64 {
        let storage = env.storage().persistent();
        
        let mut data: ClaimsData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        let claimant = env.invoker();
        claimant.require_auth();
        
        let claim_id = data.next_claim_id;
        let current_time = env.ledger().timestamp();
        
        let claim = Claim {
            id: claim_id,
            policy_id,
            claimant: claimant.clone(),
            claim_amount,
            status: ClaimStatus::Pending,
            description,
            created_at: current_time,
            settled_at: 0,
            evidence,
        };
        
        let key = format_claim_key(claim_id);
        storage.set(&key, &claim);
        
        data.next_claim_id = claim_id + 1;
        data.total_claimed += claim_amount;
        storage.set(&symbol_short!("data"), &data);
        
        env.events().publish((symbol_short!("submit"), claim_id), claimant);
        
        claim_id
    }

    /// Get claim details
    pub fn get_claim(env: Env, claim_id: u64) -> Claim {
        let storage = env.storage().persistent();
        let key = format_claim_key(claim_id);
        
        storage.get(&key)
            .expect("Claim not found")
    }

    /// Approve a claim (admin only)
    pub fn approve_claim(env: Env, claim_id: u64) {
        let storage = env.storage().persistent();
        let data: ClaimsData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        data.admin.require_auth();
        
        let key = format_claim_key(claim_id);
        let mut claim: Claim = storage.get(&key)
            .expect("Claim not found");
        
        if claim.status != ClaimStatus::Pending {
            panic!("Only pending claims can be approved");
        }
        
        claim.status = ClaimStatus::Approved;
        storage.set(&key, &claim);
        
        env.events().publish((symbol_short!("approve"), claim_id), data.admin);
    }

    /// Reject a claim (admin only)
    pub fn reject_claim(env: Env, claim_id: u64) {
        let storage = env.storage().persistent();
        let data: ClaimsData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        data.admin.require_auth();
        
        let key = format_claim_key(claim_id);
        let mut claim: Claim = storage.get(&key)
            .expect("Claim not found");
        
        if claim.status != ClaimStatus::Pending {
            panic!("Only pending claims can be rejected");
        }
        
        claim.status = ClaimStatus::Rejected;
        storage.set(&key, &claim);
        
        env.events().publish((symbol_short!("reject"), claim_id), data.admin);
    }

    /// Settle an approved claim (admin only)
    pub fn settle_claim(env: Env, claim_id: u64) {
        let storage = env.storage().persistent();
        let mut data: ClaimsData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        data.admin.require_auth();
        
        let key = format_claim_key(claim_id);
        let mut claim: Claim = storage.get(&key)
            .expect("Claim not found");
        
        if claim.status != ClaimStatus::Approved {
            panic!("Only approved claims can be settled");
        }
        
        claim.status = ClaimStatus::Settled;
        claim.settled_at = env.ledger().timestamp();
        
        storage.set(&key, &claim);
        
        data.total_settled += claim.claim_amount;
        storage.set(&symbol_short!("data"), &data);
        
        env.events().publish((symbol_short!("settle"), claim_id), data.admin);
    }

    /// Get claims statistics
    pub fn get_stats(env: Env) -> (u64, i128, i128) {
        let storage = env.storage().persistent();
        let data: ClaimsData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        (data.next_claim_id - 1, data.total_claimed, data.total_settled)
    }
}

fn format_claim_key(claim_id: u64) -> soroban_sdk::Symbol {
    soroban_sdk::symbol_short!("clm")
}
