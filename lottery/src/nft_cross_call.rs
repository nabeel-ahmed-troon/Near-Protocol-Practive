use crate::*;
use near_sdk::{ext_contract, log, Gas, PromiseOrValue, PromiseResult,AccountId};
pub type TokenId=String;


#[ext_contract(nft_contract)]
pub trait NFTCONTRACT{
        fn nft_transfer(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            //we introduce an approval ID so that people with that approval ID can transfer the token
            approval_id: Option<u64>,
            memo: Option<String>,
        );
}

pub trait LOTTERYCONTRACT{
    fn claim_reward(&self,);
}

#[near_bindgen]
impl LOTTERYCONTRACT for Lottery{

     fn claim_reward(&self,){
         require!(self.lottery_state==LOTTERYSTATE::CLAIMREWARD,"Cannot claimed yet");
        // nft_contract::nft_transfer(self.winner.clone(), self.winning_nft_token_id.clone(), Some("NFT for lottery winner".to_string()), "lotteri.testnet".to_string().parse().unwrap(), 0, Gas(5_000_000_000_000));
        nft_contract::nft_transfer(env::predecessor_account_id(), "14112".to_string(), None, None, "example-nft.testnet".to_string().parse().unwrap(), 1, Gas(5000000000000));
    }
}