#[cfg(test)]
mod tests{
    use crate::solution::Solution;

    #[test]
    pub fn test_1920() {
        assert_eq!(Solution::build_array(vec![5,0,1,2,3,4]), vec![4,5,0,1,2,3]);
        assert_eq!(Solution::build_array(vec![0,2,1,5,3,4]), vec![0,1,2,4,5,3])
    }
}