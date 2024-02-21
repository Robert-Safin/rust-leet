//Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

//You must implement a solution with a linear runtime complexity and use only constant extra space.

pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut nums = nums;
  nums.sort_unstable(); // Sorting the array
  let mut i = 0;
  while i < nums.len() {
      // If we're at the last element or the current element is not equal to the next one,
      // we've found the single number.
      if i == nums.len() - 1 || nums[i] != nums[i + 1] {
          return nums[i];
      }
      i += 2; // Skip the next element since it's a duplicate
  };
  unreachable!("There should be a single number according to the problem statement.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }
    fn test_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    fn test_3() {
        assert_eq!(single_number(vec![1]), 1);
    }
}
