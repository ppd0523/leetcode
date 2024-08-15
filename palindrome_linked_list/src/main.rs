// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
    fn from(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for e in vec.into_iter().rev() {
            let mut node = Box::new(ListNode::new(e));
            node.next = current;
            current = Some(node);
        }

        dbg!(current)
    }
}

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut res: Vec<i32> = Vec::new();
        let mut current = &head;
        while let Some(ref node) = current {
            res.push((*node).val);
            current = &node.next;
        }

        res == res.iter().rev().copied().collect::<Vec<i32>>()
    }
}

fn main() {
    let in1 = ListNode::from(vec![1,2,2,1]);
    let out1 = Solution::is_palindrome(in1);
    dbg!(out1);

    let in2 = ListNode::from(vec![1,2]);
    let out2 = Solution::is_palindrome(in2);
    dbg!(out2);
}