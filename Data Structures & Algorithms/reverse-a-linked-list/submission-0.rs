// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut acc: Vec<i32> = vec![];
        Self::collect(head, &mut acc);

        let mut result = None;
        for val in acc {
            result = Some(Box::new(ListNode { val, next: result}));
        }
        result
    }

    fn collect(head: Option<Box<ListNode>>, acc: &mut Vec<i32>) {
        if let Some(x) = head {
            acc.push(x.val);
            Self::collect(x.next, acc);
        }
    }
}
