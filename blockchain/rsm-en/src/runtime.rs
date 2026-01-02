use crate::{balances, system, traits::Config};

#[derive(Debug)]
pub struct RunTime<T:Config>{
  pub  balance :balances::Pallet<T>, 
  pub system : system::Pallet<T>,
}

impl <T:Config>   RunTime<T> {
    pub fn new() -> Self {
        Self {
            balance: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

}