use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Integer, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Nonce, BlockNumber> {
    block_number: BlockNumber,  // number of blocks . 
    nonce: BTreeMap<AccountId , Nonce>, // count of transactions per account
}


impl<AccountId, Nonce, BlockNumber> Pallet<AccountId, Nonce, BlockNumber> where 

AccountId : Ord + Clone,
Nonce : Copy + CheckedAdd + Zero + CheckedSub + Integer,
BlockNumber : Copy + CheckedAdd + Zero + CheckedSub + Integer,

{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn set_block_number(&mut self, number: BlockNumber) {
        self.block_number = number;
    }

    pub fn inc_block_number(&mut self) {
        // crashes on overflow 
        self.block_number = self.block_number.checked_add(&BlockNumber::one()).unwrap();
    }

    pub fn get_block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_nonce(&mut self, account: &AccountId) {
        let current_nonce = *self.nonce.get(account).unwrap_or(&Nonce::zero());
        self.nonce.insert(account.clone(), current_nonce + Nonce::one());
    }

    pub fn get_nonce(&self, account: &AccountId) -> Nonce {
        *self.nonce.get(account).unwrap_or(&Nonce::zero())
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