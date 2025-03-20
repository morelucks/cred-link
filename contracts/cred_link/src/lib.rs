#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env, Map, Vec};

//... Constants....//
const MIN_CREDIT_SCORE: u32 = 300;
const MAX_CREDIT_SCORE: u32 = 850;
const DEFAULT_CREDIT_SCORE: u32 = 500;

const STABLECOIN_RATIO: u32 = 11000; // 110%
const XLM_RATIO: u32 = 12500; // 125%
const OTHER_ASSET_RATIO: u32 = 15000; // 150%

const LIQUIDATION_THRESHOLD: u32 = 10500; // 105%
const WARNING_THRESHOLD: u32 = 11000; // 110%
//....Data Structures....//
#[contracttype]
pub struct UserProfile {
    pub stellar_address: Address,
    pub credit_score: u32,
    pub total_loans_completed: u32,
    pub total_loans_defaulted: u32,
    pub on_time_payments: u32,
    pub late_payments: u32,
    pub transaction_history: Vec<TransactionRecord>,
    pub registration_date: u64,
    pub identity_verified: bool,
}

#[contracttype]
pub struct TransactionRecord {
    pub tx_hash: Bytes,
    pub timestamp: u64,
    pub tx_type: TransactionType,
    pub amount: u64,
    pub asset: Bytes,
}

#[contracttype]
pub enum TransactionType {
    Payment,
    Loan,
    Repayment,
    Liquidation,
}

#[contracttype]
pub struct LendingPool {
    pub asset: Bytes,
    pub total_funds: u64,
    pub available_funds: u64,
    pub min_credit_score: u32,
    pub interest_rate: u32,     // basis points (100 = 1%)
    pub max_loan_duration: u32, // in days
}

#[contracttype]
pub struct Loan {
    pub id: Bytes,
    pub borrower: Address,
    pub lender: Address,
    pub amount: u64,
    pub asset: Bytes,
    pub collateral_amount: u64,
    pub collateral_asset: Bytes,
    pub interest_rate: u32, // basis points (100 = 1%)
    pub start_date: u64,
    pub due_date: u64,
    pub status: LoanStatus,
}

#[contracttype]
pub enum LoanStatus {
    Active,
    Completed,
    Defaulted,
    Liquidated,
}

//....Contract State....//
#[contract]
pub struct CredLinkContract;

#[contractimpl]
impl CredLinkContract {
    //.... Register a new user....//
    pub fn register_user(env: Env, stellar_address: Address) {
        let mut users: Map<Address, UserProfile> =
            env.storage().instance().get(&"users").unwrap_or(Map::new(&env));

        if users.contains_key(stellar_address) {
            panic!("User already exists");
        }

        let new_user = UserProfile {
            stellar_address:env.current_contract_address(),
            credit_score: DEFAULT_CREDIT_SCORE,
            total_loans_completed: 0,
            total_loans_defaulted: 0,
            on_time_payments: 0,
            late_payments: 0,
            transaction_history: Vec::new(&env),
            registration_date: env.ledger().timestamp(),
            identity_verified: false,
        };

        // users.set(stellar_address, new_user);

        env.storage().instance().set(&"users", &users);
    }

    //.... Create a lending pool....//
    pub fn create_lending_pool(
        env: Env,
        asset: Bytes,
        initial_funds: u64,
        interest_rate: u32,
        min_credit_score: u32,
        max_loan_duration: u32,
    ) {
        let mut lending_pools: Map<Bytes, LendingPool> = env
            .storage().instance()
            .get(&"lending_pools")
            .unwrap_or(Map::new(&env));

        if lending_pools.contains_key(asset) {
            panic!("Lending pool already exists");
        }

        let new_pool = LendingPool {
            asset: asset,
            total_funds: initial_funds,
            available_funds: initial_funds,
            min_credit_score,
            interest_rate,
            max_loan_duration,
        };

        lending_pools.set(asset, new_pool);
        env.storage().instance().set(&"lending_pools", &lending_pools);
    }

   
   
}
