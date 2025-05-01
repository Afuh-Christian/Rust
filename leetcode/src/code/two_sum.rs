

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut hash_map = std::collections::HashMap::new();
  
    for (i , &num) in nums.iter().enumerate(){
      if let Some(&val) = hash_map.get(&(target - num)){
          return vec![val as i32 , i as i32]
      }
      hash_map.insert(num , i);
    }
  vec![]
  }
  