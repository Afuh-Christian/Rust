use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
nums.sort(); // Sort the vector first
nums.dedup(); // Remove consecutive duplicates
nums.len() as i32 // Return the new length
}



pub fn remove_duplicates_(nums: &mut Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    nums.retain(|x| seen.insert(*x));
    nums.len() as i32
}

// retain() keeps only the first occurrence of each number.

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}


pub fn remove_element_(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    for j in 0..nums.len(){
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i); // Cut off the extra elements
    i as i32
}
