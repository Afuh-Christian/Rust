use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::traits::Config;




#[derive(Debug)]
pub struct Pallet<T:Config> {
     balances : BTreeMap<T::AccountId, T::Balance>,
}


impl <T:Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: &T::AccountId, amount: T::Balance) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn balance(&self, account: &T::AccountId) -> T::Balance {   
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(&mut self, from: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        
        let from_balance = self.balance(&from).checked_sub(&amount).ok_or("Insufficient balance")?;
        let to_balance = self.balance(&to).checked_add(&amount).ok_or("Overflow")?;

   
     self.set_balance(&from, from_balance);
     self.set_balance(&to, to_balance);


     Ok(())
    }



}