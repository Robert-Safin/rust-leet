/*
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than
⌊n / 2⌋ times. You may assume that the majority element always
exists in the array.
*/

use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for n in nums.iter() {
        if hash.contains_key(&n) {
            *hash.get_mut(&n).unwrap() += 1;
        } else {
            hash.insert(*n, 1);
        }
    }
    let highest = hash.iter().find(|(_, v)| nums.len() as i32 / 2 < **v);
    *highest.unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
