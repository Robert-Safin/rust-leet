/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing
order, and two integers m and n, representing the number of elements in
nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead
be stored inside the array nums1. To accommodate this, nums1 has a length
of m + n, where the first m elements denote the elements that should be
merged, and the last n elements are set to 0 and should be ignored. nums2
has a length of n.
*/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut new: Vec<i32> = Vec::new();

    for i in 0..m {
        new.push(nums1[i as usize])
    }

    for i in 0..n {
        new.push(nums2[i as usize])
    }

    new.sort();

    *nums1 = new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
