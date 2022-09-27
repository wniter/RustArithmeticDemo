use super::util::linked_list::{ListNode, to_list};


pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = merge_two_lists(n, Some(node2));
                Some(node1)
            } else {
                let n = node2.next.take();
                node2.next = merge_two_lists(Some(node1), n);
                Some(node2)
            }
        },
        _ => None,
    }
}