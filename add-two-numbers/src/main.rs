// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut c = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut c;
        while l1.is_some() || l2.is_some() {
            let a = l1.take().unwrap_or(Box::new(ListNode::new(0)));
            let b = l2.take().unwrap_or(Box::new(ListNode::new(0)));
            tail = match tail.as_mut() {
                Some(inner_box) => {
                    let sum = a.val + b.val + inner_box.val;
                    let carry = sum / 10;
                    inner_box.val = sum % 10;
                    inner_box.next = if a.next.is_none() && b.next.is_none() && carry == 0 {
                        None
                    } else {
                        Some(Box::new(ListNode::new(carry)))
                    };
                    &mut inner_box.next
                }
                _ => unreachable!(),
            };
            l1 = a.next;
            l2 = b.next;
        }
        c
    }
}
fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(3))),
            })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));
    println!("{:#?}", Solution::add_two_numbers(l1, l2));
}
