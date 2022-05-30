use near_sdk::require;
use crate::*;
impl Lottery{
    pub(crate) fn only_owner(&self){
        require!(env::predecessor_account_id()==self.owner,"Only Owner Can Call This Function");
    }
}
