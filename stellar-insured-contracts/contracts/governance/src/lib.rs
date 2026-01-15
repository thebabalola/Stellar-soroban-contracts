#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Executed,
    Cancelled,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: Address,
    pub yes_votes: i128,
    pub no_votes: i128,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub end_time: u64,
    pub threshold_percentage: i128,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct GovernanceData {
    pub next_proposal_id: u64,
    pub admin: Address,
    pub voting_period_days: u64,
    pub min_voting_percentage: i128,
    pub token_contract: Address,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Initialize the governance contract
    pub fn initialize(
        env: Env,
        admin: Address,
        token_contract: Address,
        voting_period_days: u64,
        min_voting_percentage: i128,
    ) {
        let storage = env.storage().persistent();
        
        admin.require_auth();
        
        let data = GovernanceData {
            next_proposal_id: 1,
            admin: admin.clone(),
            voting_period_days,
            min_voting_percentage,
            token_contract,
        };
        
        storage.set(&symbol_short!("gov"), &data);
    }

    /// Create a new governance proposal
    pub fn create_proposal(
        env: Env,
        title: String,
        description: String,
        threshold_percentage: i128,
    ) -> u64 {
        let storage = env.storage().persistent();
        
        let proposer = env.invoker();
        proposer.require_auth();
        
        let mut data: GovernanceData = storage.get(&symbol_short!("gov"))
            .expect("Governance not initialized");
        
        let proposal_id = data.next_proposal_id;
        let current_time = env.ledger().timestamp();
        let end_time = current_time + (data.voting_period_days * 86400);
        
        let proposal = Proposal {
            id: proposal_id,
            title,
            description,
            proposer: proposer.clone(),
            yes_votes: 0,
            no_votes: 0,
            status: ProposalStatus::Active,
            created_at: current_time,
            end_time,
            threshold_percentage,
        };
        
        let key = format_proposal_key(proposal_id);
        storage.set(&key, &proposal);
        
        data.next_proposal_id = proposal_id + 1;
        storage.set(&symbol_short!("gov"), &data);
        
        env.events().publish((symbol_short!("prop"), proposal_id), proposer);
        
        proposal_id
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        let storage = env.storage().persistent();
        let key = format_proposal_key(proposal_id);
        
        storage.get(&key)
            .expect("Proposal not found")
    }

    /// Vote on a proposal
    pub fn vote(env: Env, proposal_id: u64, vote_weight: i128, is_yes: bool) {
        let storage = env.storage().persistent();
        
        let voter = env.invoker();
        voter.require_auth();
        
        let key = format_proposal_key(proposal_id);
        let mut proposal: Proposal = storage.get(&key)
            .expect("Proposal not found");
        
        let current_time = env.ledger().timestamp();
        
        if current_time > proposal.end_time {
            panic!("Voting period has ended");
        }
        
        if proposal.status != ProposalStatus::Active {
            panic!("Proposal is not active");
        }
        
        if is_yes {
            proposal.yes_votes += vote_weight;
        } else {
            proposal.no_votes += vote_weight;
        }
        
        let vote_key = format_vote_key(&voter, proposal_id);
        storage.set(&vote_key, &(is_yes, vote_weight));
        
        storage.set(&key, &proposal);
        
        env.events().publish((symbol_short!("vote"), proposal_id), voter);
    }

    /// Finalize a proposal after voting period
    pub fn finalize_proposal(env: Env, proposal_id: u64) {
        let storage = env.storage().persistent();
        
        let key = format_proposal_key(proposal_id);
        let mut proposal: Proposal = storage.get(&key)
            .expect("Proposal not found");
        
        let current_time = env.ledger().timestamp();
        
        if current_time <= proposal.end_time {
            panic!("Voting period has not ended yet");
        }
        
        let total_votes = proposal.yes_votes + proposal.no_votes;
        
        if total_votes == 0 {
            proposal.status = ProposalStatus::Failed;
        } else {
            let yes_percentage = (proposal.yes_votes * 10000) / total_votes;
            
            if yes_percentage >= proposal.threshold_percentage {
                proposal.status = ProposalStatus::Passed;
            } else {
                proposal.status = ProposalStatus::Failed;
            }
        }
        
        storage.set(&key, &proposal);
        
        env.events().publish((symbol_short!("final"), proposal_id), proposal.proposer);
    }

    /// Get governance statistics
    pub fn get_stats(env: Env) -> u64 {
        let storage = env.storage().persistent();
        let data: GovernanceData = storage.get(&symbol_short!("gov"))
            .expect("Governance not initialized");
        
        data.next_proposal_id - 1
    }
}

fn format_proposal_key(proposal_id: u64) -> soroban_sdk::Symbol {
    soroban_sdk::symbol_short!("prop")
}

fn format_vote_key(voter: &Address, proposal_id: u64) -> soroban_sdk::Symbol {
    soroban_sdk::symbol_short!("vote")
}
