use std::cmp::{max, min};

use crate::solution::Solution;

#[derive(Debug)]
struct MedianSlice<'a> {
    pub value: f64,
    pub lower_index: usize,
    pub upper_index: usize,
    pub slice: &'a [i32],
}

impl PartialEq for MedianSlice<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd for MedianSlice<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for MedianSlice<'_> {}

impl Ord for MedianSlice<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.value.total_cmp(&other.value);
    }
}

impl Solution {
    fn is_even(number: usize) -> bool {
        return (number % 2) == 0;
    }

    #[inline]
    fn median(slice: &[i32]) -> MedianSlice {
        let slice_len = slice.len();
        let median_index = slice_len / 2;
        match Self::is_even(slice_len) {
            true => {
                return MedianSlice {
                    value: ((slice[median_index] + slice[median_index - 1])
                        as f64
                        / 2_f64),
                    lower_index: median_index - 1,
                    upper_index: median_index,
                    slice,
                };
            }
            false => {
                return MedianSlice {
                    value: slice[median_index] as f64,
                    lower_index: median_index,
                    upper_index: median_index,
                    slice,
                };
            }
        }
    }

    #[inline]
    fn search_step<'a>(
        left: &'a [i32],
        right: &'a [i32],
    ) -> (&'a [i32], &'a [i32]) {
        let left_median = Self::median(left);
        let right_median = Self::median(right);

        let lesser_median = min(&left_median, &right_median);
        let greater_median = max(&left_median, &right_median);

        let offset = max(
            min(
                lesser_median.lower_index,
                greater_median.slice.len() - (greater_median.lower_index + 1),
            ) - 1,
            1,
        );

        return (
            &lesser_median.slice[offset..],
            &greater_median.slice[..greater_median.slice.len() - offset],
        );
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut lesser_slice = nums1.as_slice();
        let mut greater_slice = nums2.as_slice();

        while lesser_slice.len() > 2 && greater_slice.len() > 2 {
            (lesser_slice, greater_slice) =
                Self::search_step(lesser_slice, greater_slice);
            println!("{:?} \n{:?} \n", lesser_slice, greater_slice);
        }

        let mut final_combined_slice = [lesser_slice, greater_slice].concat();

        final_combined_slice.sort();

        return Self::median(&final_combined_slice).value;
    }
}

#[test]
fn it_works() {
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(vec![2, 2, 4, 4], vec![2, 2, 4, 4])
    );
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(vec![2, 3, 5, 7], vec![1, 4, 6])
    );
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6])
    );
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6, 7])
    );
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6, 7, 8])
    );
    println!(
        "{}\n",
        Solution::find_median_sorted_arrays(
            vec![2, 3, 4, 7, 9, 10],
            vec![1, 5, 6, 8]
        )
    );
}
