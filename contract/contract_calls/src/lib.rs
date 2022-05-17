use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
// use near_sdk::PromiseOrValue;
use near_sdk::{env,near_bindgen,ext_contract,AccountId,PromiseOrValue,PanicOnDefault,log,Gas};
use near_sdk::json_types::{U128,ValidAccountId};
const BASE_GAS: Gas = Gas(100_000_000_000_000);
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract{

}
pub trait ContractMethods{
    fn purchase(account_id:AccountId);
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ft_contract)]
trait FungibleTokenTrait{
     fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}


#[near_bindgen]
impl ContractMethods for Contract{
    fn purchase(_account_id:AccountId){
        let _amount= U128::from(10);
        let _msg= "Lottery".to_string();
        let _memo=Some("memo".to_string());
        let _current_contract= env::current_account_id();
        log!("Inside purchase fn");
        ft_contract::ft_transfer_call(_current_contract, _amount, _memo, _msg, _account_id, 1, BASE_GAS);
    }
    fn ft_on_transfer(  &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String
    ) -> PromiseOrValue<U128>{
        let a:u128 = 0;
        log!("{:?} received from {} ",amount,sender_id);
        near_sdk::PromiseOrValue::Value(U128::from(a))
    }
}




