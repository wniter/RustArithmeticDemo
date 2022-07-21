pub mod _linkedlist;
// use super::_linkedlist::util::linked_list::{ListNode, to_list};
use self::_linkedlist::util::linked_list::{to_list, ListNode};
use _linkedlist::linked_list_cycle::has_cycle;
use _linkedlist::merge_two_sorted_lists::merge_two_lists;

fn main() {
    println!("{:?}", has_cycle(to_list(vec![1, 2, 3, 4, 5])));

    println!(
        "{:?}",
        merge_two_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4]))
    );
}
