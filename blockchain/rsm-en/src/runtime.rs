use crate::{ balances, proof_of_existence, support::{self, Dispatch}, system, types};


// pub enum RuntimeCall{
//   Balances(balances::Call<Runtime>),
//   ProofOfExistence(proof_of_existence::Call<Runtime>),
// }



impl system::Config for Runtime {
  type AccountId = types::AccountId;
  type BlockNumber = types::BlockNumber;
  type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
  type AccountId = types::AccountId;
  type Balance = types::Balance;
  type BlockNumber = types::BlockNumber;
}

impl proof_of_existence::Config for Runtime {
  type Content = types::Content;
}




#[derive(Debug)]

#[macros::runtime]
pub struct Runtime{
  pub system : system::Pallet<Runtime>,
  pub  balances :balances::Pallet<Runtime>, 
  pub proof_of_existence : proof_of_existence::Pallet<Runtime>,
}


