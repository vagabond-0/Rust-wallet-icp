use std::{cell::RefCell, collections::HashMap};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use candid::{CandidType, Principal};

type Persons = HashMap<Principal, User>;
type Idstore = HashMap<String, Principal>;

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, Default)]
pub struct User {
    pub username: String,
    pub balance: u64,
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone, Default)]
pub struct State {
    pub users: Persons,
    pub id_store: Idstore,
    pub total_supply: u64,
    pub balances: HashMap<Principal, u64>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[query]
pub fn get_self() -> User {
    let id = ic_cdk::api::caller();
    STATE.with(|state| {
        state.borrow()
            .users
            .get(&id)
            .cloned()
            .unwrap_or_default()
    })
}

#[update]
pub fn create_account(user: User) -> Option<User> {
    let id = ic_cdk::caller();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        if let Some(existing_principal) = state.id_store.get(&user.username) {
            return state.users.get(existing_principal).cloned();
        }
        
        state.id_store.insert(user.username.clone(), id);
        state.users.insert(id, User {
            username: user.username.clone(),
            balance: 1000,
        });
        state.balances.insert(id, 1000);
        
        Some(User {
            username: user.username,
            balance: 1000,
        })
    })
}

#[query]
pub fn get_balance() -> u64 {
    let user_id = ic_cdk::caller();
    STATE.with(|state| {
        state.borrow().balances.get(&user_id).cloned().unwrap_or(0)
    })
}

#[update]
pub fn transfer_tokens(to: Principal, amount: u64) -> Result<(), String> {
    let from = ic_cdk::caller();
    
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let from_balance = state.balances.get(&from).cloned().unwrap_or(0);
        
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        *state.balances.entry(from).or_insert(0) -= amount;
        *state.balances.entry(to).or_insert(0) += amount;
        
        Ok(())
    })
}

ic_cdk::export_candid!();