
use crate::{runtime::{RunTime, RuntimeCall}, traits::Config};
mod traits;
mod balances;
mod runtime;
mod system;
mod support;



mod types {

    use crate::{runtime::RuntimeCall, support};

  pub type AccountId = String;
  pub type Balance = u128;
  pub type BlockNumber = u32;
  pub type Nonce = u32;
  pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
  pub type Header = support::Header<BlockNumber>;
  pub type Block = support::Block<Header, Extrinsic>;
  // TODO: Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`.
  // TODO: Define a concrete `Header` type using `BlockNumber`.
  // TODO: Define a concrete `Block` type using `Header` and `Extrinsic`.
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


// Define a block with two extrinsics:
let block_1 = types::Block {
  header: support::Header { block_number: 1 },
  extrinsics: vec![

    // transfer 69 from alice to bob
    support::Extrinsic {
      caller: alice.clone(),
      call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
    },

     // transfer 30 from bob to charlie
       support::Extrinsic {
      caller: alice.clone(),
      call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 }),
    },
  ],
};

// Define a block with two extrinsics:
let block_2 = types::Block {
  header: support::Header { block_number: 2 },
  extrinsics: vec![

    // transfer 69 from alice to bob
    support::Extrinsic {
      caller: alice.clone(),
      call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 30 }),
    },

     // transfer 30 from bob to charlie
       support::Extrinsic {
      caller: alice.clone(),
      call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20 }),
    },

        // transfer 30 from bob to charlie
       support::Extrinsic {
      caller: bob.clone(),
      call: RuntimeCall::Balances(balances::Call::Transfer { to: alice.clone(), amount: 25 }),
    },
  ],
};

   runtime.execute_block(block_1).expect("panic!");
   runtime.execute_block(block_2).expect("panic!");

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
