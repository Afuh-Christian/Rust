use crate::{ balances, proof_of_existence, support::{self, Dispatch}, system, types};


pub enum RuntimeCall{
  Balances(balances::Call<RunTime>),
  ProofOfExistence(proof_of_existence::Call<RunTime>),
}





impl system::Config for RunTime {
  type AccountId = types::AccountId;
  type BlockNumber = types::BlockNumber;
  type Nonce = types::Nonce;
}

impl balances::Config for RunTime {
  type AccountId = types::AccountId;
  type Balance = types::Balance;
  type BlockNumber = types::BlockNumber;
}

impl proof_of_existence::Config for RunTime {
  type Content = types::Content;
}




#[derive(Debug)]
pub struct RunTime{
  pub  balance :balances::Pallet<RunTime>, 
  pub system : system::Pallet<RunTime>,
  pub proof_of_existence : proof_of_existence::Pallet<RunTime>,
}




impl RunTime {
    pub fn new() -> Self {
        Self {
            balance: balances::Pallet::new(),
            system: system::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
pub fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {



  // TODO:
  // - Increment the system's block number.
  self.system.inc_block_number();

  // - Check that the block number of the incoming block matches the current block number,
  //   or return an error.
    if  self.system.get_block_number() != block.header.block_number  {
        return Err("Block number mismatch");
    }
  // - Iterate over the extrinsics in the block.
  //   - Increment the nonce of the caller.
  //   - Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
  //   - Handle errors from `dispatch` same as we did for individual calls: printing any
  //     error and capturing the result.
  // - You can extend the error message to include information like the block number
  //   and extrinsic number.


  for (i , support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
  self.system.inc_nonce(&caller);
  let _ = self.dispatch(caller, call)
    .inspect_err(|e| {
        eprintln!(
    "Extrinsic Error\n\tBlock Number: {}\
    \n\tExtrinsic Number: {}\n\tError: {}",
    block.header.block_number, 1, e
    );
    });

}
  
  Ok(())
// }

}
}





//also ADD THIS CODE TO YOUR main.rs file:
impl crate::support::Dispatch for RunTime {
  type Caller = <RunTime as system::Config>::AccountId;
  type Call = RuntimeCall;

  // Dispatch a call on behalf of a caller. Increments the caller's nonce.
  //
  // Dispatch allows us to identify which underlying module call we want to execute.
  // Note that we extract the `caller` from the extrinsic, and use that information
  // to determine who we are executing the call on behalf of.
  fn dispatch(
    &mut self,
    caller: Self::Caller,
    runtime_call: Self::Call,
  ) -> support::DispatchResult {

   match runtime_call {
      RuntimeCall::Balances(call) =>  {self.balance.dispatch(caller, call)?;}
      RuntimeCall::ProofOfExistence(call) =>  {self.proof_of_existence.dispatch(caller, call)?;}
   }

   Ok(())
}
}