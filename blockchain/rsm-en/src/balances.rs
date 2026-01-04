use std::{collections::BTreeMap, fmt::Debug, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, Integer, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone + Debug;
    type Balance: Clone + Copy + CheckedAdd + CheckedSub + Integer + Debug;
    type BlockNumber: Clone
        + Copy
        + AddAssign
        + One
        + Zero
        + CheckedAdd
        + CheckedSub
        + Integer
        + Debug;
}



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

    pub fn transfer(&mut self, caller: &T::AccountId, to: &T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        
        let from_balance = self.balance(&caller.clone()).checked_sub(&amount).ok_or("Insufficient balance")?;
        let to_balance = self.balance(&to.clone()).checked_add(&amount).ok_or("Overflow")?;

        self.set_balance(&caller.clone(), from_balance);
        self.set_balance(&to.clone(), to_balance);

        Ok(())
    }



}