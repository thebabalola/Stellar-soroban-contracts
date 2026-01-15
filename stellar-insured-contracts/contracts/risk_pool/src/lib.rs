#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short};

#[derive(Clone, Debug)]
#[contracttype]
pub struct RiskPoolData {
    pub total_liquidity: i128,
    pub available_liquidity: i128,
    pub total_providers: u64,
    pub admin: Address,
    pub xlm_token: Address,
    pub min_provider_stake: i128,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct LiquidityProvider {
    pub address: Address,
    pub stake_amount: i128,
    pub contribution_time: u64,
    pub share_percentage: i128,
}

#[contract]
pub struct RiskPoolContract;

#[contractimpl]
impl RiskPoolContract {
    /// Initialize the risk pool contract
    pub fn initialize(
        env: Env,
        admin: Address,
        xlm_token: Address,
        min_provider_stake: i128,
    ) {
        let storage = env.storage().persistent();
        
        admin.require_auth();
        
        let data = RiskPoolData {
            total_liquidity: 0,
            available_liquidity: 0,
            total_providers: 0,
            admin: admin.clone(),
            xlm_token,
            min_provider_stake,
        };
        
        storage.set(&symbol_short!("pool"), &data);
    }

    /// Deposit liquidity into the risk pool
    pub fn deposit_liquidity(env: Env, provider: Address, amount: i128) {
        let storage = env.storage().persistent();
        
        provider.require_auth();
        
        if amount < storage.get::<_, RiskPoolData>(&symbol_short!("pool"))
            .expect("Pool not initialized")
            .min_provider_stake
        {
            panic!("Stake amount below minimum");
        }
        
        let mut data: RiskPoolData = storage.get(&symbol_short!("pool"))
            .expect("Pool not initialized");
        
        let mut liquidity_provider: Option<LiquidityProvider> = 
            storage.get(&format_provider_key(&provider));
        
        match liquidity_provider {
            Some(mut lp) => {
                lp.stake_amount += amount;
                lp.share_percentage = (lp.stake_amount * 10000) / (data.total_liquidity + amount);
                storage.set(&format_provider_key(&provider), &lp);
            }
            None => {
                let new_provider = LiquidityProvider {
                    address: provider.clone(),
                    stake_amount: amount,
                    contribution_time: env.ledger().timestamp(),
                    share_percentage: if data.total_liquidity == 0 {
                        10000
                    } else {
                        (amount * 10000) / (data.total_liquidity + amount)
                    },
                };
                storage.set(&format_provider_key(&provider), &new_provider);
                data.total_providers += 1;
            }
        }
        
        data.total_liquidity += amount;
        data.available_liquidity += amount;
        storage.set(&symbol_short!("pool"), &data);
        
        env.events().publish((symbol_short!("deposi"), 1), provider);
    }

    /// Withdraw liquidity from the risk pool
    pub fn withdraw_liquidity(env: Env, provider: Address, amount: i128) {
        let storage = env.storage().persistent();
        
        provider.require_auth();
        
        let mut data: RiskPoolData = storage.get(&symbol_short!("pool"))
            .expect("Pool not initialized");
        
        if data.available_liquidity < amount {
            panic!("Insufficient available liquidity");
        }
        
        let mut liquidity_provider: LiquidityProvider = storage.get(&format_provider_key(&provider))
            .expect("Provider not found");
        
        if liquidity_provider.stake_amount < amount {
            panic!("Provider insufficient balance");
        }
        
        liquidity_provider.stake_amount -= amount;
        
        if liquidity_provider.stake_amount == 0 {
            storage.remove(&format_provider_key(&provider));
            data.total_providers -= 1;
        } else {
            liquidity_provider.share_percentage = 
                (liquidity_provider.stake_amount * 10000) / (data.total_liquidity - amount);
            storage.set(&format_provider_key(&provider), &liquidity_provider);
        }
        
        data.total_liquidity -= amount;
        data.available_liquidity -= amount;
        storage.set(&symbol_short!("pool"), &data);
        
        env.events().publish((symbol_short!("witdra"), 1), provider);
    }

    /// Get pool statistics
    pub fn get_pool_stats(env: Env) -> (i128, i128, u64) {
        let storage = env.storage().persistent();
        let data: RiskPoolData = storage.get(&symbol_short!("pool"))
            .expect("Pool not initialized");
        
        (data.total_liquidity, data.available_liquidity, data.total_providers)
    }

    /// Get provider details
    pub fn get_provider_info(env: Env, provider: Address) -> (i128, i128) {
        let storage = env.storage().persistent();
        let lp: LiquidityProvider = storage.get(&format_provider_key(&provider))
            .expect("Provider not found");
        
        (lp.stake_amount, lp.share_percentage)
    }

    /// Reserve liquidity for claim settlement (admin only)
    pub fn reserve_liquidity(env: Env, amount: i128) {
        let storage = env.storage().persistent();
        let mut data: RiskPoolData = storage.get(&symbol_short!("pool"))
            .expect("Pool not initialized");
        
        data.admin.require_auth();
        
        if data.available_liquidity < amount {
            panic!("Insufficient liquidity for reservation");
        }
        
        data.available_liquidity -= amount;
        storage.set(&symbol_short!("pool"), &data);
        
        env.events().publish((symbol_short!("reserv"), 1), data.admin);
    }

    /// Release reserved liquidity (admin only)
    pub fn release_liquidity(env: Env, amount: i128) {
        let storage = env.storage().persistent();
        let mut data: RiskPoolData = storage.get(&symbol_short!("pool"))
            .expect("Pool not initialized");
        
        data.admin.require_auth();
        
        let reserved = data.total_liquidity - data.available_liquidity;
        
        if reserved < amount {
            panic!("Cannot release more than reserved");
        }
        
        data.available_liquidity += amount;
        storage.set(&symbol_short!("pool"), &data);
        
        env.events().publish((symbol_short!("releas"), 1), data.admin);
    }
}

fn format_provider_key(provider: &Address) -> soroban_sdk::Symbol {
    soroban_sdk::symbol_short!("prov")
}
