use crate::*;
use near_sdk::{ext_contract, PromiseOrValue};
pub type TokenId=String;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
// use near_contract_standards::non_fungible_token::Token;

#[ext_contract(nft_contract)]
pub trait NFTCONTRACT{
        fn nft_transfer_call(
            &mut self,
            receiver_id: AccountId,
            token_id: TokenId,
            approval_id: Option<u64>,
            memo: Option<String>,
            msg: String,
        ) -> PromiseOrValue<bool>;

        fn nft_mint(
            &mut self,
            token_id: TokenId,
            receiver_id: AccountId,
            token_metadata: TokenMetadata,
        ) -> Token;
}
trait FTActionsReceiver {

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

pub trait LOTTERYCONTRACT{
    fn claim_reward(&mut self,_token_id: TokenId);
}

#[near_bindgen]
impl LOTTERYCONTRACT for Lottery{

     fn claim_reward(&mut self,_token_id: TokenId){
         //Assertion Claimer should be winner
         assert!(env::predecessor_account_id()==self.winner,"Only {} can Claim Reward",self.winner);
         //Check Lottery State
         require!(self.lottery_state==LOTTERYSTATE::CLAIMREWARD,"Cannot claimed yet");
         
        let _token_metadata= TokenMetadata{
            title: Some("Lottery Winning NFT".to_string()),
            description:Some("You Won this NFT from Out Lottery System in which you entered through custom FT".to_string()),
            media: Some("https://i.pinimg.com/originals/e0/3e/db/e03edbe588d3866d539e5bbb35d9080c.jpg".to_string()),
            media_hash:None,
            copies:None,
            issued_at:None,
            expires_at:None,
            starts_at:None,
            updated_at:None,
            extra:None,
            reference:None,
            reference_hash:None
        };
         //Minting Nft 
         nft_contract::nft_mint(_token_id, self.winner.clone(), _token_metadata, "lottery_nft.testnet".to_string().parse().unwrap(), 0, BASE_GAS)
         .then(
             ext_self::callback_promise_result(env::current_account_id(), 0, BASE_GAS)
            );
        self.lottery_state=LOTTERYSTATE::CLOSED;
    }
}

#[near_bindgen]
impl FTActionsReceiver for Lottery{
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>{
        require!(self.lottery_state==LOTTERYSTATE::OPEN,"Lottery is on Different State OR Tickets Sold.");
        assert!(
            self.approved_ft==env::predecessor_account_id(),
            "Only approved FT can be staked"
        );
        assert!(self.ticket_id<= self.ticket_limit,"All Ticket Sold");
        let expected_amount= self.ticket_price;
        assert!(amount==expected_amount,"Ticket Price is 1 BKRT Token");
        
        self.players.push(sender_id);
        self.ticket_id+=1;
        log!("Account Id : {}",env::predecessor_account_id());
        log!("Ticket No : {}",self.ticket_id);
        if self.ticket_id>=self.ticket_limit{
            self.lottery_state=LOTTERYSTATE::CALCULATINGWINNER;
        }
        near_sdk::PromiseOrValue::Value(U128::from(0))
    }
}