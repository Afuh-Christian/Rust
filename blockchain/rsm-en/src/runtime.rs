use crate::{balances, system, types::AccountId};

#[derive(Debug)]
pub struct RunTime{
  pub  balance : balances::Pallet , 
    pub system : system::Pallet,
}

impl RunTime {
    pub fn new() -> Self {
        Self {
            balance: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

}