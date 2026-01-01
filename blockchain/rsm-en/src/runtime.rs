use num::{CheckedAdd, CheckedSub, Integer, Zero};

use crate::{balances, system};

#[derive(Debug)]
pub struct RunTime<AccountId , Nonce , Balance , BlockNumber>{
  pub  balance : balances::Pallet<AccountId, Balance>, 
    pub system : system::Pallet<AccountId, Nonce, BlockNumber>,
}

impl <AccountId , Nonce , Balance , BlockNumber>   RunTime<AccountId , Nonce , Balance , BlockNumber> where 
    AccountId : Ord + Clone,
    Nonce : Copy + CheckedAdd + Zero + CheckedSub + Integer,
    Balance : Copy + CheckedAdd + Zero + CheckedSub + Integer,
    BlockNumber : Copy + CheckedAdd + Zero + CheckedSub + Integer,
{
    pub fn new() -> Self {
        Self {
            balance: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

}