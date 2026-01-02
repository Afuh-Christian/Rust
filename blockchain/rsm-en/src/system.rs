use std::{collections::BTreeMap, fmt::Debug, ops::AddAssign};
use num::{CheckedAdd, CheckedSub, Zero,Integer,One};

// use crate::traits::{Config};

pub trait Config {
    type AccountId: Ord + Clone + Debug;
    type Nonce: Clone + Copy + CheckedAdd + CheckedSub + Integer + Debug;
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
    block_number: T::BlockNumber,  // number of blocks . 
    nonce: BTreeMap<T::AccountId , T::Nonce>, // count of transactions per account
}


impl<T:Config> Pallet<T>
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn set_block_number(&mut self, number: T::BlockNumber) {
        self.block_number = number;
    }

    pub fn inc_block_number(&mut self) {
        // crashes on overflow 
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_nonce(&mut self, account: &T::AccountId) {
        let current_nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(account.clone(), current_nonce + T::Nonce::one());
    }

    pub fn get_nonce(&self, account: &T::AccountId) -> T::Nonce {
        *self.nonce.get(account).unwrap_or(&T::Nonce::zero())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_block_number() {
    //     let mut pallet = Pallet::new();
    //     assert_eq!(pallet.get_block_number(), 0);
    //     pallet.set_block_number(5);
    //     assert_eq!(pallet.get_block_number(), 5);
    //     pallet.inc_block_number();
    //     assert_eq!(pallet.get_block_number(), 6);
    // }

    // #[test]
    // fn test_nonce() {
    //     let mut pallet = Pallet::new();
    //     let account = "Alice".to_string();
    //     assert_eq!(pallet.get_nonce(&account), 0);
    //     pallet.inc_nonce(&account);
    //     assert_eq!(pallet.get_nonce(&account), 1);
    //     pallet.inc_nonce(&account);
    //     assert_eq!(pallet.get_nonce(&account), 2);
    // }


    // #[test]
    // fn test_nonce_different_accounts() {
    //     let mut pallet = Pallet::new();
    //     let account1 = "Bob".to_string();
    //     let account2 = "Charlie".to_string();
    //     assert_eq!(pallet.get_nonce(&account1), 0);
    //     assert_eq!(pallet.get_nonce(&account2), 0);
    //     pallet.inc_nonce(&account1);
    //     assert_eq!(pallet.get_nonce(&account1), 1);
    //     assert_eq!(pallet.get_nonce(&account2), 0);
    // }

    // // write test for incrementing nonce and block number for multiple accounts
    // #[test]
    // fn test_nonce_multiple_accounts() {
    //     let mut pallet = Pallet::new();
    //     let account1 = "Bob".to_string();
    //     let account2 = "Charlie".to_string();
    //     assert_eq!(pallet.get_nonce(&account1), 0);
    //     assert_eq!(pallet.get_nonce(&account2), 0);
    //     pallet.inc_nonce(&account1);
    //     pallet.inc_nonce(&account2);
    //     assert_eq!(pallet.get_nonce(&account1), 1);
    //     assert_eq!(pallet.get_nonce(&account2), 1);
    // }

}