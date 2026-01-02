
use crate::{runtime::RunTime, traits::Config};
mod traits;
mod balances;
mod runtime;
mod system;
mod support;



mod types {

    use crate::support;

  pub type AccountId = String;
  pub type Balance = u128;
  pub type BlockNumber = u32;
  pub type Nonce = u32;
  pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
  pub type Header = support::Header<BlockNumber>;
  pub type Block = support::Block<Header, Extrinsic>;
  // TODO: Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`.
  // TODO: Define a concrete `Header` type using `BlockNumber`.
  // TODO: Define a concrete `Block` type using `Header` and `Extrinsic`.
}





pub enum RuntimeCall{

}






#[derive(Debug)]
pub struct  TestConfig;

impl Config for TestConfig {
    type AccountId = String;
    type Nonce = u64;
    type Balance = u64;
    type BlockNumber = u64;
}

fn main() {

    let mut runtime: RunTime = RunTime::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();
    let charlie: String = "charlie".to_string();

    runtime.balance.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.get_block_number(), 1);
     
    // first transaction
         runtime.system.inc_nonce(&alice);
    let _ =  runtime.balance.transfer(&alice, &bob, 30) 
    .map_err(|e| println!("Error : {:?}" , e));

    // second transaction
         runtime.system.inc_nonce(&alice);
    let _ =  runtime.balance.transfer(&alice, &charlie, 30) 
    .map_err(|e| println!("Error : {:?}" , e));

    println!("Runtime State: {:#?}", runtime);

}







#[test]
fn test_runtime() {
    let mut runtime : RunTime = RunTime::new();

    let alice: String = "alice".to_string();
    let bob: String = "bob".to_string();
    let charlie: String = "charlie".to_string();

    runtime.balance.set_balance(&alice, 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.get_block_number(), 1);

    // first transaction
    runtime.balance.transfer(&alice, &bob, 30).unwrap();
    runtime.system.inc_nonce(&alice);
    runtime.system.inc_block_number();

    assert_eq!(runtime.balance.balance(&alice), 70);
    assert_eq!(runtime.balance.balance(&bob), 30);
    assert_eq!(runtime.system.get_nonce(&alice), 1);
    assert_eq!(runtime.system.get_block_number(), 2);
}
