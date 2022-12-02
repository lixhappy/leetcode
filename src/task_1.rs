use crate::Solution;
use std::collections::HashMap;

// 1. Two Sum
// https://leetcode.com/problems/two-sum/
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // before 
        // let mut hash: HashMap<i32, i32> = HashMap::new();

        // after
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        // if the exact max size is know, 
        // it's better to use 
        // HashMap::with_capacity(size) 
        // for initialization

        for (i2, num) in nums.into_iter().enumerate() {
            match hash.get(&(target - num)) {
                Some(i1) => { return vec![*i1, i2 as i32] }
                None => { hash.insert(num, i2 as i32); }
            }
        }
        // dead code {
        vec![]
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0,1]);

        assert_eq!(Solution::two_sum(vec![3, -10, 100, 1, -13, 0, -110], -10), vec![0,4]);
    }
}

