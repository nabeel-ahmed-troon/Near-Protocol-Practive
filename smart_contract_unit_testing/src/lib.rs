/* All the imports involving serialization are used to bundle
// the code/storage so that it's ready for the blockchain.
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Operation {
    pub num1: u128,
    pub num2: u128,
    //In hashmap String => UserName, u128=>UserBalance
    pub map: HashMap<String, u128>,
    //String=>UserName, Vec(u128=>UserIds)
    pub map_with_vec: HashMap<String, Vec<String>>,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            num1: 10,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        }
    }
}
//This macro is used to make struct compatible wity Near blockchain
#[near_bindgen]
impl Operation {
    /*******************/
    /* CHANGE METHODS */
    /******************/
    // Arguments:
    // * `a`: First number which you want to add.
    // * `b`: Second number which you want to add

    pub fn add(&mut self, a: u128, b: u128) -> u128 {
        self.num1 = a;
        self.num2 = b;
        let c = self.num1 + self.num2;

        env::log(b"After Addition");
        c
    }
    // Arguments:
    // * `a`: First number which you wanted to perform subtraction.
    // * `b`: Second number which you wanted to perform subtraction
    pub fn sub(&mut self, a: u128, b: u128) -> u128 {
        self.num1 = a;
        self.num2 = b;
        let c = self.num1 - self.num2;

        env::log(b"After Substraction");
        c
    }
    // Arguments:
    // * `a`: First number which you wanted to perform multiplication.
    // * `b`: Second number which you wanted to perform multiplication
    // * returns result
    pub fn mul(&mut self, a: u128, b: u128) -> u128 {
        self.num1 = a;
        self.num2 = b;
        let c = self.num1 * self.num2;

        env::log(b"After multiplication");
        c
    }
    /// Arguments:
    // * `k`: Here 'k' is a key which will sote in mapping/hashmap.
    // * `v`: Value of vectors, will be stored in the mapping/hashmap corresponding to the key.

    pub fn insertion_with_vector(&mut self, k: String, v: Vec<String>) {
        self.map_with_vec.insert(k, v);
    }

    pub fn deletion_with_vector(&mut self, k: &String) -> () {
        if let Some(value) = self.map_with_vec.remove(&k.to_string()) {
            Some(&value)
        } else {
            None
        };
        let c = env::log(b"Succesfully Deleted");
        c
    }

    pub fn get_map_vector(&self, k: String) -> Option<u128> {
        if let Some(value) = self.map.get(&k) {
            Some(*value)
        } else {
            None
        }
    }

    pub fn delletion_of_value_within_vector(&mut self, k: String, v: String) {
        let mut vec = self.map_with_vec.get(&k).unwrap().to_vec();

        let selected_index = self
            .map_with_vec
            .get(&k)
            .unwrap()
            .to_vec()
            .iter()
            .position(|i| i == &v.to_string());

        vec.remove(selected_index.unwrap());

        self.map_with_vec.insert(k, vec);
    }

    /// Arguments:
    // * `k`: Here 'k' is a key which will sote in mapping/hashmap.
    // * `v`: Value of type u128, will be stored in the mapping/hashmap corresponding to the key.

    pub fn insertion(&mut self, k: String, v: u128) {
        self.map.insert(k.clone(), v.clone());
    }
    /// Arguments:
    // * `k`: Here 'k' is a key which is sotered in mapping/hashmap.

    pub fn delete(&mut self, k: String) {
        // self.map.remove(&k);
        if let Some(value) = self.map.remove(&k) {
            Some(&value)
        } else {
            None
        };
        env::log(b"deleting the map");
    }

    /****************/
    /* VIEW METHODS */
    /****************/
    /// Arguments:
    // * `k`: Here 'k' is a key which will sote in mapping/hashmap.

    pub fn get_hash_with_vector(&self, k: String) -> Option<&Vec<String>> {
        if let Some(value) = self.map_with_vec.get(&k.to_string()) {
            Some(&value)
        } else {
            None
        }
    }

    pub fn read(&self, k: String) -> Option<&u128> {
        env::log(b"Get Balance");

        if let Some(value) = self.map.get(&k.to_string().clone()) {
            Some(&value)
        } else {
            None
        }
    }

    //To retrive the vector length in a Mapping
    pub fn get_vec_size(&self, k: String) -> usize {
        self.map_with_vec.get(&k).unwrap().to_vec().len()
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
    #[ignore]
    fn test1() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };
        let c: Vec<String> = vec!["Green".to_string(), "Blue".to_string()];

        contract.insertion_with_vector("nabeel".to_string(), c);

        contract.get_hash_with_vector("nabeel".to_string());
    }
    #[test]

    fn addition_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };
        //Numbers to be added
        let a = 10;
        let b = 10;
        //Perform addition, storing in 'result'
        let result = contract.add(a, b);

        log!("Additoin occur");
        //comparing expecting value with real value
        assert_eq!(20, result, "We Added {} and {} = {}", a, b, result);
    }
    #[test]

    fn substraction_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };

        let a = 10;
        let b = 10;
        //Perform substraction, storing in 'result'
        let result = contract.sub(a, b);

        log!("Additoin occur");
        //comparing expecting value with real value
        assert_eq!(0, result, "We Added {} and {} = {}", a, b, result);
    }
    #[test]

    fn multiplication_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };

        let a = 10;
        let b = 10;
        //Perform multiplicaton, storing in 'result'
        let c = contract.mul(a, b);

        log!("Additoin occur");
        //Comparing results
        assert_eq!(100, c, "We Added {} and {} = {}", a, b, c);
    }
    #[test]

    fn insert_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };
        //Key of Map
        let key = String::from("nabeelahmed.testnet");
        //Value of Map
        let value: u128 = 100;
        //insertion to map
        let _c = contract.insertion(key, value);
        //After insertion, we expecting 10 in mapping
        let expecting_value: Option<u128> = Some(100);
        //Comparing
        assert_eq!(
            expecting_value,
            contract.get_map_vector("nabeelahmed.testnet".to_string())
        );
    }

    #[test]

    fn delete_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };
        //Key of mapping
        let key = String::from("nabeelahmed.testnet");
        //Its Value
        let value: u128 = 100;
        //Insertion
        let _c = contract.insertion(key, value);

        let expecting_value: Option<u128> = Some(100);
        //Checking that Value inserted succesfully
        assert_eq!(
            expecting_value,
            contract.get_map_vector("nabeelahmed.testnet".to_string())
        );
        //Delete Key from mapping
        contract.delete("nabeelahmed.testnet".to_string());
        //Comapring (value should be None after deletion)
        assert_eq!(
            None,
            contract.get_map_vector("nabeelahmed.testnet".to_string())
        );
    }

    #[test]

    fn update_test() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };

        let key = String::from("nabeelahmed.testnet");
        let value: u128 = 100;
        let _c = contract.insertion(key.clone(), value.clone());

        let expecting_value: Option<u128> = Some(100);
        //Comparing
        assert_eq!(
            expecting_value,
            contract.get_map_vector("nabeelahmed.testnet".to_string())
        );

        let key = String::from("nabeelahmed.testnet");
        let value: u128 = 110;
        let expecting_value: Option<u128> = Some(110);
        let _c = contract.insertion(key, value);
        assert_eq!(
            expecting_value,
            contract.get_map_vector("nabeelahmed.testnet".to_string())
        );
    }
    #[test]
    pub fn delete_element_of_vector_from_mapping() {
        let context = get_context("alice.testnet".to_string(), 5000);
        testing_env!(context);
        //Create a Object of Struct
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };

        //Insertion of Values into the mapping
        let _c = contract
            .map_with_vec
            .insert("nabeel".to_string(), vec!["a".to_string(), "b".to_string()]);
        //Value we are expecting after insertion
        let get = vec!["a".to_string(), "b".to_string()];
        //Comparing expecting value with actual value of mapping
        assert_eq!(
            &get,
            contract.get_hash_with_vector("nabeel".to_string()).unwrap()
        );
        //Deletion of element from the mapping
        contract.delletion_of_value_within_vector("nabeel".to_string(), "a".to_string());
        //Mapping key accros which vector is stored
        let key = "nabeel".to_string();
        //After Deletion getting the remaining elements from vector
        let get1 = contract.get_hash_with_vector(key).unwrap();
        //Comparing previous get value with new get value (should not equal)
        assert_ne!(&get, get1);
    }
    /***********************/
    /* VIEW METHODS TESTING*/
    /***********************/
    #[test]
    pub fn get_length_of_vector() {
        let context = get_context("alice.testnet".to_string(), 50000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };
        //Vector which will be inserted in mapping
        let a: Vec<String> = vec!["a".to_string(), "b".to_string()];
        //Insetion in mapping
        contract.insertion_with_vector("nabeel".to_string(), a);
        //expecting 2, after inseting two value in vector
        assert_eq!(2, contract.get_vec_size("nabeel".to_string()));
    }
    #[test]
    pub fn get_value_of_vector() {
        let context = get_context("alice.testnet".to_string(), 5000);
        testing_env!(context);
        let mut contract = Operation {
            num1: 19,
            num2: 20,
            map: HashMap::new(),
            map_with_vec: HashMap::new(),
        };

        let a: Vec<String> = vec!["a".to_string(), "b".to_string()];
        contract.insertion_with_vector("nabeel".to_string(), a);
        //getting value from hashmap and storing in b
        let b = contract.get_hash_with_vector("nabeel".to_string());
        // should be true if b has correct value
        assert!(b == contract.get_hash_with_vector("nabeel".to_string()));
    }
}
