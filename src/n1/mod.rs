/*
Given an integer array nums sorted in non-decreasing order, remove
the duplicates in-place such that each unique element appears only
once. The relative order of the elements should be kept the same.
Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get
accepted, you need to do the following things:

Change the array nums such that the first k elements of nums
contain the unique elements in the order they were present in
nums initially. The remaining elements of nums are not important
as well as the size of nums.

Return k.
*/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  println!("{:?}", nums);
    let mut unique_index = 0;
    let mut search_index = 1;

    while search_index < nums.len() {
        if nums[unique_index] != nums[search_index] {
            unique_index += 1;
            nums[unique_index] = nums[search_index];
        }
        search_index += 1;
    }
    (unique_index + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(
            remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
