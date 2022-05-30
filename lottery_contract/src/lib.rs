use near_sdk::borsh::{self,BorshSerialize,BorshDeserialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env,BorshStorageKey,near_bindgen,AccountId,PanicOnDefault, require,log,PromiseResult};
use near_sdk::json_types::{U128};
use near_sdk::CryptoHash;

/*
use crate::internal::*;
use crate::nft_cross_call::*;
use crate::enumeration::*;
use crate::callback::*;
*/
use crate::airdrop_ft::*;
mod airdrop_ft;
mod nft_cross_call;
mod internal;
mod enumeration;
mod callback;

#[derive(Debug)]
#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
pub struct Lottery{
    owner: AccountId,
    players: Vec<AccountId>,
    winner: AccountId,
    ticket_price: U128,
    ticket_limit: u64,
    ticket_id: u64,
    lottery_state: LOTTERYSTATE,
    winner_picked: bool,
    winning_nft_token_id: u128,
    approved_ft:AccountId,
    //For Airdrop of Ft
    claimers: UnorderedMap<AccountId,U128>,
    claimers_count:u128,
    airdrop_amount:U128,
    airdrop_revoke: bool,
    nft_account:AccountId
}
#[derive(Debug)]
#[derive(BorshDeserialize,BorshSerialize,)]
#[derive(PartialEq)]
enum LOTTERYSTATE{
    OPEN,
    CLOSED,
    CALCULATINGWINNER,
    CLAIMREWARD,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys{
    CLAIMERS
}

#[near_bindgen]
impl Lottery{
    //Contract Initializaton
    #[init]
    pub fn new()->Self{
        let _this = Self{
            owner: env::predecessor_account_id(),
            players: Vec::new(),
            winner: "unknown".to_string().parse().unwrap(),
            ticket_price: U128::from(1),
            ticket_limit: 10,
            ticket_id:0,
            lottery_state:LOTTERYSTATE::CLOSED,
            winner_picked: false,
            winning_nft_token_id: 1,
            approved_ft: "lottery_ft.testnet".to_string().parse().unwrap(),
            //Ft Airdrop
            claimers: UnorderedMap::new(StorageKeys::CLAIMERS),
            claimers_count:0,
            airdrop_amount:U128::from(5000000000000000000000000),
            airdrop_revoke: false,
            nft_account:"lottery_nft.testnet".to_string().parse().unwrap(),
        };
        _this
    }

    pub fn start_new_lottery(&mut self,ticket_limit:u64,ticket_price:U128,approved_ft:AccountId,nft_contract:AccountId){
        require!(self.owner==env::predecessor_account_id(),"Only owner can Start Lottery");
        require!(self.lottery_state==LOTTERYSTATE::CLOSED,"Previous Lottery is Not Ended.");    
        self.ticket_id=0;
        self.ticket_limit=ticket_limit;
        self.ticket_price=ticket_price;
        self.approved_ft=approved_ft;
        self.lottery_state= LOTTERYSTATE::OPEN;
        self.nft_account=nft_contract.clone();

        if self.winner_picked==true{
            self.players=Vec::new();
            self.ticket_id=0;
            self.nft_account=nft_contract;
        }

    }

    pub fn pick_winner(&mut self){
        //Checking Lottery State
        require!(self.lottery_state==LOTTERYSTATE::CALCULATINGWINNER,"Lottery State is not Calculating Winner");
        //Should be Owner
        self.only_owner();
        //Randomness
        let mut seed: Vec<u8> = vec![];
        seed.extend(env::random_seed());
        let mut randomness: [u8; 32] = CryptoHash::default();
        randomness.copy_from_slice(&env::sha256(seed.as_slice()));
        log!("Random Number is {:?}",randomness[2]);
        let s = self.players.len();
        log!("Players length: {}",s);
        //Selecting Winner
        let selected_index=(randomness[2]%(s as u8)) as usize;
        let winner=(self.players.get(selected_index)).unwrap();
        log!("Players of index{}: Winner is {:?}",selected_index,winner);
        self.winner=winner.clone();
        self.winner_picked=true;
        //Change State to Claim Reward
        self.lottery_state=LOTTERYSTATE::CLAIMREWARD;
    }

    

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    
    // TESTS HERE
    #[test]
    fn init_lottery(){
        let context= get_context(accounts(1));
        testing_env!(context.build());
        let contract=Lottery::new();
        println!("{:#?}",contract);
        assert!(contract.winner_picked==false);
        assert!(contract.lottery_state==LOTTERYSTATE::CLOSED);
    }

    #[test]
    fn start_new_lottery(){
        let context= get_context(accounts(1));
        testing_env!(context.build());
        let mut contract=Lottery::new();
        let con1=contract.ticket_price;
        println!("New : {:#?}",contract);
        contract.start_new_lottery(10, U128::from(2000000000000000000000000), "ft.testnet".to_string().parse().unwrap());
        let con2=contract.ticket_price;
        assert_ne!(con1,con2);
    }

    #[test]
    #[should_panic(expected="Airdroping is Stopped by owner")]
    fn revoke_airdrop(){
        let context= get_context(accounts(1));
        testing_env!(context.build());
        let mut contract=Lottery::new();
        //checking airdrop is revoked or not
        assert!(contract.airdrop_revoke==false);
        //revoking airdrop
        contract.airdrop_revoke=true;
        //checking airdrop rovoked or not
        assert!(contract.airdrop_revoke==true);
        //trying for airdrop
        contract.ft_airdrop();
        

    }
    #[test]
    fn resume_airdrop(){
        let context= get_context(accounts(1));
        testing_env!(context.build());
        let mut contract=Lottery::new();
        //checking airdrop is revoked or not
        assert!(contract.airdrop_revoke==false);
        //revoking airdrop
        contract.airdrop_revoke=true;
        //checking airdrop rovoked or not
        assert!(contract.airdrop_revoke==true);
        //resuming airdrop
        contract.airdrop_revoke=false;
        assert!(contract.airdrop_revoke==false);
        //trying for airdrop
        contract.ft_airdrop();
        

    }




    

    


}
