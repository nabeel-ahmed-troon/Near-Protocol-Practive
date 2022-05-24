use crate::*;
use near_sdk::{Gas, ONE_YOCTO, Promise};
use near_sdk::{ext_contract,env,PromiseResult};

pub const BASE_GAS: Gas = Gas(5_000_000_000_000);
#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_promise_result() -> bool;
}

#[ext_contract(ext_lottery_fungible_token)]
pub trait FungibleTokenContract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

pub trait Airdrop{
    fn ft_airdrop(&mut self)->Promise;
    fn get_airdrop_claimers(&self)->Vec<(AccountId,U128)>;
}
#[near_bindgen]
impl Airdrop for Lottery{
    
    fn ft_airdrop(&mut self)->Promise{
        let result=self.claimers.get(&env::predecessor_account_id()).unwrap();
        assert!(!(u128::from(result)>0)||panic!("called `Option::unwrap()` on a `None` value"),"Already Claimed");
        let _amount=U128::from(5000000000000000000000000);
        // Promise::
        //Transfering FT
       let t= ext_lottery_fungible_token::ft_transfer(env::predecessor_account_id(), _amount, Some("Airdrop for Lottery Participation".to_string()), "lottery_ft.testnet".to_string().parse().unwrap(), ONE_YOCTO,BASE_GAS).then(ext_self::callback_promise_result(env::current_account_id(), 0, BASE_GAS));

        self.claimers.insert(&env::predecessor_account_id(),&_amount );
        self.claimers_count+=1;
        t
    }
    fn get_airdrop_claimers(&self)->Vec<(AccountId,U128)>{
        let value=(&self.claimers).to_vec();
        value
    }
}

#[near_bindgen]
impl Lottery{
    #[private]
pub fn callback_promise_result(&mut self) -> bool {
    assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
    match env::promise_result(0) {
        PromiseResult::NotReady => unreachable!(),
        PromiseResult::Successful(val) => true,
        PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
    }
}
}