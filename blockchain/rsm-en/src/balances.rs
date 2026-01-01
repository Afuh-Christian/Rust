use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};




#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
     balances : BTreeMap<AccountId, Balance>,
}


impl <AccountId, Balance> Pallet<AccountId, Balance> where 
AccountId : Ord + Clone,
Balance :  CheckedAdd + CheckedSub + Zero + Copy
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &AccountId, amount: Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &AccountId) -> Balance {   
        *self.balances.get(account).unwrap_or(&Balance::zero())
    }

    pub fn transfer(&mut self, from: &AccountId, to: &AccountId, amount: Balance) -> Result<(), &'static str> {
        
        let from_balance = self.balance(&from).checked_sub(&amount).ok_or("Insufficient balance")?;
        let to_balance = self.balance(&to).checked_add(&amount).ok_or("Overflow")?;

   
     self.set_balance(&from, from_balance);
     self.set_balance(&to, to_balance);


     Ok(())
    }



}