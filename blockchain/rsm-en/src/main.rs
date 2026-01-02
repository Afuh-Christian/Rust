
use crate::{runtime::RunTime, traits::Config};
mod traits;
mod balances;
mod runtime;
mod system;


#[derive(Debug)]
pub struct  TestConfig;

impl Config for TestConfig {
    type AccountId = String;
    type Nonce = u64;
    type Balance = u64;
    type BlockNumber = u64;
}

fn main() {

    let mut runtime: RunTime<TestConfig> = RunTime::new();

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
    let mut runtime : RunTime<TestConfig> = RunTime::new();

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
