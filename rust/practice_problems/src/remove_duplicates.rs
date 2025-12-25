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
}
pub struct Solution;
impl Solution {
    pub fn delete_duplicates_from_linked_list(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next;

            // Skip duplicates
            while let Some(ref next) = current {
                if next.val == node.val {
                    current = current.take().unwrap().next;
                } else {
                    break;
                }
            }

            // Add unique node to result
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "Remove duplicates in rust is not working currently"]
    fn test_delete_duplicates() {
        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(1)));
        head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(3)));

        let mut expected = ListNode::new(1);
        expected.next = Some(Box::new(ListNode::new(2)));
        expected.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        assert_eq!(
            Solution::delete_duplicates_from_linked_list(Some(Box::new(head))),
            Some(Box::new(expected))
        );
    }
}
