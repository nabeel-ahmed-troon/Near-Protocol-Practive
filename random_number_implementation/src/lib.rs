use near_sdk::borsh::{self,BorshDeserialize,BorshSerialize};
use near_sdk::{near_bindgen,PanicOnDefault,env};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Lottery{
    rand: Vec<u8>
}

#[near_bindgen]
impl Lottery {
    #[private]
    fn generate_random_number(&mut self)->Vec<u8>{
        let random= env::random_seed();
        random
        // self.rand=String::from_utf8(random.clone()).unwrap();
        // self.rand.clone()
       
    }
    fn get_random_num(&self)->Vec<u8>{
        let result=self.rand.clone();
        result
    }

}

#[cfg(test)]
mod tests {

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

        let mut contract= Lottery{
            rand:Vec::new()
        };

        // contract.generate_random_number();
        let result= contract.get_random_num();
        // log!("Random Nuber : {}", result);
        println!("Random{:#?}",result);

    }
}
