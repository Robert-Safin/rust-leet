/*
Given an array of integers nums and an integer target, return
indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution,
and you may not use the same element twice.

You can return the answer in any order.
*/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = Vec::new();

    'outer: for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                indices.push(i as i32);
                indices.push(j as i32);
                break 'outer;
            }
        }
    }

    indices
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
