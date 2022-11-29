
pub struct Solution;

impl Solution {

    // 1920. Build Array from Permutation
    // https://leetcode.com/problems/build-array-from-permutation/
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in nums.iter() {
            let tmp = nums[*i as usize];
            ans.push(tmp);
        }
        ans
    }
}
