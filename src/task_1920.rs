
use crate::Solution;
use crate::OtherSolution;

// 1920. Build Array from Permutation
// https://leetcode.com/problems/build-array-from-permutation/

impl Solution {

    // my solution
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in nums.iter() {
            // crutch
            let tmp = nums[*i as usize];
            ans.push(tmp);
        }
        ans
    }
}

impl OtherSolution {

    // solutions from 
    // https://leetcode.com/problems/build-array-from-permutation/solutions/1317383/rust-solutions/?languageTags=rust
    // memory O(n) runtime ~4ms
    pub fn build_array_1(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()) // Iterator Range<usize>
        .map(|i| nums[nums[i] as usize]) // Iterator
        .collect() // make Vector from Iterator
    }

    // memory O(1) runtime ~4ms
    pub fn build_array_2(mut nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len() as i32;
        (0..l as usize).for_each(|i| nums[i] += (nums[nums[i] as usize] % l) * l);
        (0..l as usize).for_each(|i| nums[i] /= l);
        nums
    }
    
    // with bit operation 
    // memory O(1) runtime ~1ms
    // It's very interested
    pub fn build_array_3(mut nums: Vec<i32>) -> Vec<i32> {
        let mask = 1023;
        (0..nums.len()).for_each(|i| nums[i] |= (nums[nums[i] as usize] & mask) << 10);
        (0..nums.len()).for_each(|i| nums[i] >>= 10);
        nums
    }


}


#[cfg(test)]
mod tests{
    use crate::{Solution, OtherSolution};

    

    #[test]
    pub fn test_1920() {
        assert_eq!(Solution::build_array(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
        assert_eq!(Solution::build_array(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3]);
    }

    #[test]
    pub fn test_other_1920_1() {
        assert_eq!(OtherSolution::build_array_1(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
        assert_eq!(OtherSolution::build_array_1(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3]);
    }

    #[test]
    pub fn test_other_1920_2() {
        assert_eq!(OtherSolution::build_array_2(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
        assert_eq!(OtherSolution::build_array_2(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3]);
    }

    #[test]
    pub fn test_other_1920_3() {
        assert_eq!(OtherSolution::build_array_3(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
        assert_eq!(OtherSolution::build_array_3(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3]);
    }
}