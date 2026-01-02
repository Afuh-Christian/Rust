use crate::{balances, support, system  , types};





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



#[derive(Debug)]
pub struct RunTime{
  pub  balance :balances::Pallet<RunTime>, 
  pub system : system::Pallet<RunTime>,
}

impl RunTime {
    pub fn new() -> Self {
        Self {
            balance: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
self.system.inc_block_number();


    if  self.system.get_block_number() != block.header.block_number  {
        return Err("Block number mismatch");
    }


  // TODO:
  // - Increment the system's block number.
  // - Check that the block number of the incoming block matches the current block number,
  //   or return an error.
  // - Iterate over the extrinsics in the block.
  //   - Increment the nonce of the caller.
  //   - Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
  //   - Handle errors from `dispatch` same as we did for individual calls: printing any
  //     error and capturing the result.
  // - You can extend the error message to include information like the block number
  //   and extrinsic number.
  
  Ok(())
}

}