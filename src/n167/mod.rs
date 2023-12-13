/*
Given a 1-indexed array of integers numbers that is already
sorted in non-decreasing order, find two numbers such that they
add up to a specific target number. Let these two numbers be
numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.

Return the indices of the two numbers, index1 and index2, added by
one as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution.
You may not use the same element twice.

Your solution must use only constant extra space.


*/

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut indices = Vec::new();

    'outer: for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == target {
                indices.push((i + 1) as i32);
                indices.push((j + 1) as i32);
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
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
    #[test]
    fn test_2() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }
    #[test]
    fn test_3() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
