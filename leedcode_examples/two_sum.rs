// // Very fast and memory efficient code....but a little  comlex for biggeners
// use std::collections::HashMap;
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut num_map = HashMap::new();

//     for (i, &num) in nums.iter().enumerate() {
//         let complement = target - num;

//         if let Some(&complement_index) = num_map.get(&complement) {
//             println!("indices {:?} ", vec![complement_index as i32, i as i32]);
//             return vec![complement_index as i32, i as i32];
//         }
//         num_map.insert(num, i);
//     }

//     vec![]
// }

// Slow  and memory inefficient code....but pretty straight forward for biggeners to understand
fn two_sum(nums: Vec<i32>, _target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i..nums.len() - 1 {
            if nums[i] + nums[j + 1] == _target {
                println!("indices {:?} ", vec![i as i32, (j + 1) as i32]);
                return vec![i as i32, (j + 1) as i32];
            }
        }
    }
    return vec![];
}

fn main() {
    let nums = vec![5, 4, 3, 2, 1];
    let _taget: i32 = 9;
    two_sum(nums, _taget);
}
