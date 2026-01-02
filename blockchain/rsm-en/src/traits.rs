use std::{fmt::Debug, ops::AddAssign};
use num::{CheckedAdd, CheckedSub, Integer, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone + Debug;
    type Nonce: Clone + Copy + CheckedAdd + CheckedSub + Integer + Debug;
    type Balance: Clone + Copy + CheckedAdd + CheckedSub + Integer + Debug;
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
