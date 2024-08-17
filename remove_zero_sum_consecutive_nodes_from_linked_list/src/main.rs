// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from(v: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;

        for e in v.iter().rev() {
            let mut node = ListNode::new(*e);
            node.next = current;
            current = Some(Box::new(node));
        }

        current
    }
}

struct Solution;
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head
    }
}

fn main() {
    let in1 = vec![1, 2, -3, 3, 1];
    let head1 = ListNode::from(&in1);
    let out1 = Solution::remove_zero_sum_sublists(head1);
    dbg!(out1);
}
