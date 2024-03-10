use crate::solution::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    #[allow(unused)]
    fn step(
        nodes: [Option<Box<ListNode>>; 2],
        carry_over: i32,
    ) -> Option<Box<ListNode>> {
        if nodes.iter().all(|item| item.is_none()) && carry_over == 0 {
            return None;
        }

        let sum: i32 = nodes
            .iter()
            .map(|item| match item {
                Some(node) => node.val,
                None => 0,
            })
            .sum::<i32>()
            + carry_over;

        let new_carry_over = if sum >= 10 { 1 } else { 0 };
        let val = sum - (new_carry_over * 10);

        let next_nodes: [Option<Box<ListNode>>; 2] = [
            nodes[0].clone().and_then(|node| node.next),
            nodes[1].clone().and_then(|node| node.next),
        ];

        return Some(Box::new(ListNode {
            val,
            next: Self::step(next_nodes, new_carry_over),
        }));
    }

    #[allow(unused)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Self::step([l1, l2], 0);
    }
}
