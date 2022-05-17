use near_sdk::borsh::{self,BorshSerialize,BorshDeserialize};
use near_sdk::{near_bindgen,PanicOnDefault,env};

#[near_bindgen]
#[derive(BorshSerialize,BorshDeserialize,PanicOnDefault)]
struct Time{
    time: u64
}

#[near_bindgen]
impl Time {
    #[init]
    pub fn new()->Self{
        let s=Self{
            time: 0
        };
        s
    }
   pub fn set_time(&mut self){
        self.time=env::block_timestamp();
    }

   pub fn get_time(&self)->u64{
       //as it is you can get block index
        self.time
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    use near_sdk::{log, MockedBlockchain};
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test1() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract= Time{
            time:0
        };

        contract.set_time();
        let res= contract.get_time();
        println!("Block Time {}",res);
    }
}
