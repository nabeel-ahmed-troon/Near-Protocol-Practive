// use near_sdk::{json_types::U128, near_bindgen, AccountId, Promise};

// #[near_bindgen]
// pub struct Contract {}

// #[near_bindgen]
// impl Contract {
//     pub fn pay(amount: U128, to: AccountId) -> Promise {
//         Promise::new(to).transfer(amount.0)
//         // Promise::new(to)
//     }
// }

use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};


const INITIAL_BALANCE: Balance = 3_000_000_000_000_000_000_000_000; // 3e24yN, 3N

#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn create_child_contract(prefix: AccountId, code: Vec<u8>) -> Promise {
        let subaccount_id = AccountId::new_unchecked(
          format!("{}.{}", prefix, env::current_account_id())
        );
        // let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc).time();
// assert_eq!(Utc.timestamp(61, 0), dt);
    
        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(code)
    }
}