use crate::*;

#[near_bindgen]
impl Lottery{

    //=========================
    //    VIEW FUNCTIONS     //
    //=========================

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

    pub fn get_ticket_price(&self)->U128{
        log!("Per Ticket Price Is : {:#?}",self.ticket_price);
        self.ticket_price
    }

    pub fn get_ticket_limit(&self)->u64{
        log!("Total Tickets Supply Is : {}",self.ticket_limit);
        self.ticket_limit
    }
    //=========================//
    //VIEW FUNTION FOR AIRDROP//
    //=========================//
    pub fn get_airdrop_claimers(&self)->Vec<(AccountId,U128)>{
        let value=(&self.claimers).to_vec();
        value
    }

    pub fn get_airdrop_count(&self)->u128{
        log!("Airdrop Count Is : {}",self.claimers_count);
        self.claimers_count
           
    }

}