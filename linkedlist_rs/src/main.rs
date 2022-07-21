pub mod _linkedlist;
// use super::_linkedlist::util::linked_list::{ListNode, to_list};
use self::_linkedlist::util::linked_list::{to_list, ListNode};
use _linkedlist::linked_list_cycle::has_cycle;
use _linkedlist::merge_two_sorted_lists::merge_two_lists;
use _linkedlist::middle_of_the_linked_list::middle_node;
use _linkedlist::remove_nth_node_from_end_of_list::remove_nth_from_end;
fn main() {
    println!("{:?}", has_cycle(to_list(vec![1, 2, 3, 4, 5])));

    println!(
        "{:?}",
        merge_two_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4]))
    );
    println!("{:?}", middle_node(to_list(vec![1, 3, 4])));

    // println!("{:?}", remove_nth_from_end(to_list(vec![1, 3, 4])));
}
