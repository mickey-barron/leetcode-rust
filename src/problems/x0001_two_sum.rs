use std::collections::HashMap;

use crate::solution::Solution;

impl Solution {
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut collector = HashMap::<i32, i32>::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(compliment_index) = collector.get(&diff) {
                return vec![index as i32, *compliment_index];
            }
            collector.insert(*num, index as i32);
        }

        return vec![0, 0];
    }
}

#[test]
fn two_sum_works() {
    fn sorted(vec: Vec<i32>) -> Vec<i32> {
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();

        return sorted_vec;
    }
    assert_eq!(sorted(Solution::two_sum(vec![1, 2], 3)), vec![0, 1]);
    assert_eq!(sorted(Solution::two_sum(vec![2, 7, -1], 6)), vec![1, 2]);
    assert_eq!(
        sorted(Solution::two_sum(vec![2, 3, 4, 5, 6], 5)),
        vec![0, 1]
    );
    assert_eq!(
        sorted(Solution::two_sum(vec![-2, 3, 4, 5, 6], 11)),
        vec![3, 4]
    );
    assert_eq!(
        sorted(Solution::two_sum(vec![-2, 3, 4, 5, 6], 2)),
        vec![0, 2]
    );
}
