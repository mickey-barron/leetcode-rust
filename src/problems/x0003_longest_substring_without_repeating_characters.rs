use crate::solution::Solution;

impl Solution {
    #[allow(unused)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut positions = [0; 128];
        let mut global_max = 0;
        let mut start_index = 0;
        for (end_index, char) in s.chars().enumerate() {
            start_index = start_index.max(positions[char as usize]);
            global_max = global_max.max(end_index as i32 - start_index + 1);
            positions[char as usize] = (end_index + 1) as i32;
        }
        return global_max;
    }
}
