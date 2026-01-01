use crate::runtime::RunTime;

mod balances;
mod runtime;
mod system;
mod types;

type RunTimeType = RunTime<String, u32, u128, u32>;

fn main() {

    let mut runtime: RunTimeType = RunTime::new();

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
    let mut runtime : RunTimeType = RunTime::new();

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
