#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::api::call;
    use candid::{Principal};
    use std::cell::RefCell;

    thread_local! {
        static STATE: RefCell<State> = RefCell::default();
    }

    fn setup() {
        STATE.with(|state| {
            state.borrow_mut().users.clear();
            state.borrow_mut().id_store.clear();
            state.borrow_mut().balances.clear();
            state.borrow_mut().total_supply = 0;
        });
    }

    #[test]
    fn test_create_account() {
        setup();

        let username = "test_user".to_string();
        let user = User {
            username: username.clone(),
            balance: 1000,
        };

        let result = create_account(user.clone());

        assert!(result.is_some());
        let created_user = result.unwrap();
        assert_eq!(created_user.username, username);
        assert_eq!(created_user.balance, 1000);

        STATE.with(|state| {
            let state = state.borrow();
            assert!(state.users.contains_key(&ic_cdk::api::caller()));
            assert_eq!(*state.balances.get(&ic_cdk::api::caller()).unwrap(), 1000);
        });
    }

    #[test]
    fn test_get_balance() {
        setup();

        let username = "test_user".to_string();
        let user = User {
            username: username.clone(),
            balance: 1000,
        };

        create_account(user.clone());

        
        let balance = get_balance();
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_transfer_tokens_success() {
        setup();

        let username1 = "user1".to_string();
        let user1 = User {
            username: username1.clone(),
            balance: 1000,
        };
        create_account(user1);

        let username2 = "user2".to_string();
        let user2 = User {
            username: username2.clone(),
            balance: 0,
        };
        create_account(user2);

        let user1_principal = ic_cdk::api::caller();

     
        let transfer_result = transfer_tokens(Principal::from_text("aaaaa-aa").unwrap(), 500);
        
      
        assert!(transfer_result.is_ok());

     
        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(*state.balances.get(&user1_principal).unwrap(), 500);
            assert_eq!(*state.balances.get(&Principal::from_text("aaaaa-aa").unwrap()).unwrap(), 500);
        });
    }

    #[test]
    fn test_transfer_tokens_insufficient_balance() {
        setup();

        let username1 = "user1".to_string();
        let user1 = User {
            username: username1.clone(),
            balance: 100,
        };
        create_account(user1);

        let username2 = "user2".to_string();
        let user2 = User {
            username: username2.clone(),
            balance: 0,
        };
        create_account(user2);

       
        let transfer_result = transfer_tokens(Principal::from_text("aaaaa-aa").unwrap(), 500);
        
     
        assert!(transfer_result.is_err());
        assert_eq!(transfer_result.err().unwrap(), "Insufficient balance".to_string());

     
        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(*state.balances.get(&ic_cdk::api::caller()).unwrap(), 100);
            assert_eq!(*state.balances.get(&Principal::from_text("aaaaa-aa").unwrap()).unwrap(), 0);
        });
    }
}
