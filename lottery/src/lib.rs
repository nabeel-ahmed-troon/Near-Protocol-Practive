use near_sdk::borsh::{self,BorshSerialize,BorshDeserialize};
use near_sdk::{env,near_bindgen,AccountId,PanicOnDefault, require,log};
// use near_sdk::collections::{Vector, LookupMap, UnorderedMap};

use crate::internal::*;
use crate::nft_cross_call::*;
mod nft_cross_call;
mod internal;

#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,PanicOnDefault)]
pub struct Lottery{
    owner: AccountId,
    players: Vec<AccountId>,
    // players: UnorderedMap<AccountId,Vec<u64>>,
    winner: AccountId,
    ticket_price: u64,
    ticket_limit: u64,
    ticket_id: u64,
    lottery_state: LOTTERYSTATE,
    winner_picked: bool,
    winning_nft_token_id: TokenId


}

#[near_bindgen]
#[derive(BorshDeserialize,BorshSerialize,)]
#[derive(PartialEq)]
enum LOTTERYSTATE{
    OPEN,
    CLOSED,
    CALCULATINGWINNER,
    CLAIMREWARD,
    WINNERCLAIMED
}
#[near_bindgen]
impl Lottery{

    #[inline]
#[init]
    pub fn new()->Self{
        let _this = Self{
            owner: env::predecessor_account_id(),
            // players: UnorderedMap::new(b"m"),
            players: Vec::new(),
            winner: "unknown".to_string().parse().unwrap(),
            ticket_price: 2,
            ticket_limit: 100,
            ticket_id:0,
            lottery_state:LOTTERYSTATE::CLOSED,
            winner_picked: false,
            winning_nft_token_id: "Token1".to_string()
          
        };
        _this
    }

    pub fn start_new_lottery(&mut self){
        require!(self.lottery_state==LOTTERYSTATE::CLOSED,"Previous lottery is not completely closed");
        require!(self.owner==env::predecessor_account_id(),"Only owner can Start Lottery");
        self.ticket_id+=1;
        self.ticket_limit=10;
        self.lottery_state= LOTTERYSTATE::OPEN;

    }

    pub fn buy_ticket(&mut self){

        require!(self.lottery_state==LOTTERYSTATE::OPEN,"New Lottery is not started yet");
        let ticket_vec = [self.ticket_id].to_vec();
        
        self.players.push(env::predecessor_account_id());
        self.ticket_id+=1;
        log!("Account Id : {}",env::predecessor_account_id());
        log!("Ticket No : {}",self.ticket_id);
        if self.ticket_id>=self.ticket_limit{
            self.lottery_state=LOTTERYSTATE::CALCULATINGWINNER;
        }
    }

    pub fn pick_winner(&mut self){
        require!(self.lottery_state==LOTTERYSTATE::CALCULATINGWINNER,"Tickets remaining");
        self.only_owner();
        let time= env::block_timestamp();
        let random= env::random_seed();
        self.lottery_state=LOTTERYSTATE::CLAIMREWARD;
    }

    pub fn get_players(&self){
        let s= &self.players;
        log!("Vector of Players : {:?}",s);
    }


    // pub fn claim_reward()
}

// impl Default for Lottery {
//     fn default() -> Self {
//         Self::new()
//     }
// }


