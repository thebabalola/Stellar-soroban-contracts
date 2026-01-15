#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum PolicyStatus {
    Active,
    Expired,
    Cancelled,
    Claimed,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Policy {
    pub id: u64,
    pub holder: Address,
    pub coverage_amount: i128,
    pub premium_amount: i128,
    pub expiry_time: u64,
    pub status: PolicyStatus,
    pub policy_type: String,
    pub created_at: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct PolicyData {
    pub next_policy_id: u64,
    pub total_premiums: i128,
    pub total_coverage: i128,
    pub admin: Address,
    pub risk_pool: Address,
}

#[contract]
pub struct PolicyContract;

#[contractimpl]
impl PolicyContract {
    /// Initialize the policy contract
    pub fn initialize(env: Env, admin: Address, risk_pool: Address) {
        let storage = env.storage().persistent();
        
        admin.require_auth();
        
        let data = PolicyData {
            next_policy_id: 1,
            total_premiums: 0,
            total_coverage: 0,
            admin: admin.clone(),
            risk_pool: risk_pool.clone(),
        };
        
        storage.set(&symbol_short!("data"), &data);
    }

    /// Issue a new insurance policy
    pub fn issue_policy(
        env: Env,
        holder: Address,
        coverage_amount: i128,
        premium_amount: i128,
        duration_days: u64,
        policy_type: String,
    ) -> u64 {
        let storage = env.storage().persistent();
        
        holder.require_auth();
        
        let mut data: PolicyData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        let policy_id = data.next_policy_id;
        let current_time = env.ledger().timestamp();
        let expiry_time = current_time + (duration_days * 86400);
        
        let policy = Policy {
            id: policy_id,
            holder: holder.clone(),
            coverage_amount,
            premium_amount,
            expiry_time,
            status: PolicyStatus::Active,
            policy_type,
            created_at: current_time,
        };
        
        // Store policy
        let key = format_policy_key(policy_id);
        storage.set(&key, &policy);
        
        // Update counters
        data.next_policy_id = policy_id + 1;
        data.total_premiums += premium_amount;
        data.total_coverage += coverage_amount;
        storage.set(&symbol_short!("data"), &data);
        
        env.events().publish((symbol_short!("issue"), policy_id), holder);
        
        policy_id
    }

    /// Get policy details
    pub fn get_policy(env: Env, policy_id: u64) -> Policy {
        let storage = env.storage().persistent();
        let key = format_policy_key(policy_id);
        
        storage.get(&key)
            .expect("Policy not found")
    }

    /// Renew a policy before expiry
    pub fn renew_policy(
        env: Env,
        policy_id: u64,
        duration_days: u64,
    ) -> u64 {
        let storage = env.storage().persistent();
        let key = format_policy_key(policy_id);
        
        let mut policy: Policy = storage.get(&key)
            .expect("Policy not found");
        
        policy.holder.require_auth();
        
        let current_time = env.ledger().timestamp();
        
        // Check if policy hasn't expired yet (grace period)
        if policy.expiry_time + 604800 < current_time {
            panic!("Policy renewal window has closed");
        }
        
        policy.expiry_time = current_time + (duration_days * 86400);
        policy.status = PolicyStatus::Active;
        
        storage.set(&key, &policy);
        
        env.events().publish((symbol_short!("renew"), policy_id), policy.holder);
        
        policy_id
    }

    /// Cancel a policy
    pub fn cancel_policy(env: Env, policy_id: u64) {
        let storage = env.storage().persistent();
        let key = format_policy_key(policy_id);
        
        let mut policy: Policy = storage.get(&key)
            .expect("Policy not found");
        
        policy.holder.require_auth();
        
        policy.status = PolicyStatus::Cancelled;
        storage.set(&key, &policy);
        
        env.events().publish((symbol_short!("cancel"), policy_id), policy.holder);
    }

    /// Mark policy as expired
    pub fn expire_policy(env: Env, policy_id: u64) {
        let storage = env.storage().persistent();
        let key = format_policy_key(policy_id);
        
        let mut policy: Policy = storage.get(&key)
            .expect("Policy not found");
        
        let current_time = env.ledger().timestamp();
        
        if policy.expiry_time >= current_time {
            panic!("Policy has not expired yet");
        }
        
        policy.status = PolicyStatus::Expired;
        storage.set(&key, &policy);
        
        env.events().publish((symbol_short!("expire"), policy_id), policy.holder);
    }

    /// Get contract statistics
    pub fn get_stats(env: Env) -> (u64, i128, i128) {
        let storage = env.storage().persistent();
        let data: PolicyData = storage.get(&symbol_short!("data"))
            .expect("Contract not initialized");
        
        (data.next_policy_id - 1, data.total_premiums, data.total_coverage)
    }
}

fn format_policy_key(policy_id: u64) -> soroban_sdk::Symbol {
    soroban_sdk::symbol_short!("pol")
}
