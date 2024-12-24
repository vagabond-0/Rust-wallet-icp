use std::{cell::RefCell, collections::HashMap};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use candid::{CandidType, Principal};

type Persons = HashMap<Principal, User>;
type Idstore = HashMap<String, Principal>;
type TokenBalance = HashMap<Principal, u64>;

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, Default)]
pub struct User {
    pub username: String,
    pub balance: u64, 
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, Default)]
pub struct IRC2Token {
    pub total_supply: u64,
    pub balances: TokenBalance,
}

thread_local! {
    static IDSTORE: RefCell<Idstore> = RefCell::default();
    static PROFILES: RefCell<Persons> = RefCell::default();
    static TOKENS: RefCell<IRC2Token> = RefCell::default();
}

#[query(name = "get self")]
pub fn get_self() -> User {
    let id = ic_cdk::api::caller();
    PROFILES.with(|profile| {
        profile
            .borrow()
            .get(&id)
            .cloned()
            .unwrap_or_default()
    })
}

#[update]
pub fn create_account(user: User) -> Option<User> {
    let id = ic_cdk::caller();

    let username_exists = IDSTORE.with(|idstore| {
        idstore.borrow().get(&user.username).cloned()
    });

    if let Some(existing_principal) = username_exists {
        return PROFILES.with(|profiles| {
            profiles.borrow().get(&existing_principal).cloned()
        });
    }

    IDSTORE.with(|idstore| {
        idstore.borrow_mut().insert(user.username.clone(), id);
    });

    PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(
            id,
            User {
                username: user.username.clone(),
                balance: 1000,
            },
        );
    });

    
    TOKENS.with(|tokens| {
        let mut token = tokens.borrow_mut();
        let user_balance = token.balances.entry(id).or_insert(0);
        *user_balance = 1000; 
    });

    Some(User {
        username: user.username,
        balance: 1000, 
    })
}

#[query]
pub fn get_balance() -> u64 {
    let user_id = ic_cdk::caller();

    TOKENS.with(|tokens| {
        let tokens = tokens.borrow();
        tokens.balances.get(&user_id).cloned().unwrap_or(0)
    })
}

#[update]
pub fn transfer_tokens(to: Principal, amount: u64) -> Result<(), String> {
    let from = ic_cdk::caller();

    TOKENS.with(|tokens| {
        let mut token = tokens.borrow_mut();

        let from_balance = token.balances.get(&from).cloned().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *token.balances.entry(from).or_insert(0) -= amount;
        *token.balances.entry(to).or_insert(0) += amount;

        Ok(())
    })
}
