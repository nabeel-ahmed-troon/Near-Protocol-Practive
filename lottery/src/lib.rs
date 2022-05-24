use near_sdk::borsh::{self,BorshSerialize,BorshDeserialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env,near_bindgen,AccountId,PanicOnDefault, require,log};
use near_sdk::json_types::{U128};
use near_sdk::CryptoHash;

// use crate::internal::*;
use crate::nft_cross_call::*;
use crate::airdrop_ft::*;
mod airdrop_ft;
mod nft_cross_call;
mod internal;

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



}

// #[near_bindgen]
// #[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
// pub struct FTAirdrop{
//     claimers: UnorderedMap<AccountId,U128>,
//     claimers_count:u128,

// }


#[derive(BorshDeserialize,BorshSerialize,)]
#[derive(PartialEq)]
enum LOTTERYSTATE{
    OPEN,
    CLOSED,
    CALCULATINGWINNER,
    CLAIMREWARD,
}
#[near_bindgen]
impl Lottery{
    //Contract Initializaton
    #[init]
    pub fn new()->Self{
        let _this = Self{
            owner: env::predecessor_account_id(),
            // players: UnorderedMap::new(b"m"),
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
            claimers: UnorderedMap::new(b"m"),
            claimers_count:0
        };
        _this
    }

    pub fn start_new_lottery(&mut self){
        require!(self.owner==env::predecessor_account_id(),"Only owner can Start Lottery");
        require!(self.lottery_state==LOTTERYSTATE::CLOSED,"Previous Lottery is Not Ended.");    
        self.ticket_id+=1;
        self.ticket_limit=10;
        self.lottery_state= LOTTERYSTATE::OPEN;

        if self.winner_picked==true{
            self.players=Vec::new();
            self.ticket_id=0;
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

    pub fn get_players(&self)->&Vec<AccountId>{
        let s= &self.players;
        log!("Vector of Players : {:?}",s);
        s
    }
    pub fn get_ticket_id(&self)->u64{
        let s=self.ticket_id;
        s
    }

    pub fn get_lottery_state(&self)->String{
        let state= &self.lottery_state;
        match state {
            LOTTERYSTATE::OPEN=>"Open".to_string(),
            LOTTERYSTATE::CLOSED=>"Closed".to_string(),
            LOTTERYSTATE::CALCULATINGWINNER=>"Calculationg Winner".to_string(),
            LOTTERYSTATE::CLAIMREWARD=>"ClaimReward".to_string(),
        }

    }


    // pub fn claim_reward()
}
// #[near_bindgen]
// impl FTAirdrop{
//     #[init]
//     fn new()->Self{
//         let this=Self{
//             claimers: UnorderedMap::new(b"m"),
//             claimers_count:0
//         };
//         this
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
