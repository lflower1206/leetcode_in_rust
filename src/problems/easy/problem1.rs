use std::collections::HashMap;
use std::convert::TryInto;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map:HashMap<&i32, i32> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    let mut index = 0;

    loop {
        let num: &i32 = &nums[index];
        let key = target - num;
        match hash_map.get(&key) {
            Some(idx) => {
                result.push(*idx);
                result.push(index.try_into().unwrap());
                break;
            },
            None => {
                hash_map.insert(num, index.try_into().unwrap());
            }
        }

        index = index + 1;
    }
    result
}
