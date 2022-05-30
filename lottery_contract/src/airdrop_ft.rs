use crate::*;
use near_sdk::{Gas, ONE_YOCTO, Promise};
use near_sdk::{ext_contract,env,near_bindgen};

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
    fn set_airdrop_amount(&mut self,amount:U128)->U128;
    fn clear_airdrop_claimers(&mut self);
    fn revoke_airdrop(&mut self);
    fn resume_airdrop(&mut self);
    fn get_approved_ft(&self)->AccountId;
}

#[near_bindgen]
impl Airdrop for Lottery{
    
     fn ft_airdrop(&mut self)->Promise{
        // let result=self.claimers.get(&env::predecessor_account_id());
      
        // assert!(!(u128::from(result)>0)||panic!("called `Option::unwrap()` on a `None` value"),"Already Claimed");
        assert!(self.airdrop_revoke==false,"Airdroping is Stopped by owner");
        if let Some(value)=self.claimers.get(&env::predecessor_account_id()){
            // Some(value);
            assert!(!(Some(value)==None));
            panic!("Already Claimed");
        }
        else{
            let _amount=self.airdrop_amount;
            // "lottery_ft.testnet".to_string().parse().unwrap()
            //=========BELOW ORIGNAL===========
            // let t= ext_lottery_fungible_token::ft_transfer(env::predecessor_account_id(), _amount, Some("Airdrop for Lottery Participation".to_string()), self.approved_ft.clone(), ONE_YOCTO,BASE_GAS)
            // .then(
            //     ext_self::callback_promise_result(env::current_account_id(), 0, BASE_GAS)
            //  );

            //=========BELOW FOR TESTING========
            let t= ext_lottery_fungible_token::ft_transfer(env::predecessor_account_id(), _amount, Some("Airdrop for Lottery Participation".to_string()), "ft".to_string().parse().unwrap(), ONE_YOCTO,BASE_GAS);
             
             self.claimers.insert(&env::predecessor_account_id(),&_amount );
             self.claimers_count+=1;
             t
        }
    }

         fn set_airdrop_amount(&mut self,amount:U128)->U128{
            self.only_owner();
            self.airdrop_amount=amount;
            self.airdrop_amount
        }
    
         fn clear_airdrop_claimers(&mut self){
            self.only_owner();
            self.claimers.clear();
        }
        
         fn revoke_airdrop(&mut self){
            self.only_owner();
            self.airdrop_revoke=true;
            log!("Airdrop is Revoke for Some Resons:")
        }
    
         fn resume_airdrop(&mut self){
            self.only_owner();
            self.airdrop_revoke=false;
            log!("Airdrop is Resumed");
        }

        fn get_approved_ft(&self)->AccountId{
           let res= &self.approved_ft; 
           res.clone()
        }
            
        }


