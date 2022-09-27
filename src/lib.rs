pub mod array;
pub mod back_tracking;
pub mod binary_search;
pub mod binary_tree;
pub mod divide_and_conquer;
pub mod dynamic_programming;
pub mod graph;
pub mod hash_table;
pub mod heap;
pub mod linkedlist;
pub mod queue;
pub mod sort;
pub mod stack;
pub mod string;
pub mod trie;
#[cfg(test)]
mod tests {
    use crate::array::array_rs;
    use crate::back_tracking::bag_exec::solve_bag;
    use crate::graph::graph_search::Graph;
    use crate::hash_table::hash_table::MyHashTable;
    use crate::heap::build_heap::{build_heap_down_up, build_heap_up_down};
    use crate::heap::get_top_k;
    use crate::heap::heap::Heap;
    use crate::heap::merge_sorted_array::merge_sorted_array;
    use crate::heap::{self, get_median::get_median, sort_heap};
    use crate::linkedlist;
    use crate::string::bf_rk::{bf, rk};
    use crate::string::bm::bm_search;
    use crate::string::kmp::kmp_search;
    use crate::trie::trie::Trie;

    // use super::_linkedlist::util::linked_list::{ListNode, to_list};
    use linkedlist::linked_list_cycle::has_cycle;
    use linkedlist::merge_two_sorted_lists::merge_two_lists;
    use linkedlist::middle_of_the_linked_list::middle_node;
    use linkedlist::remove_nth_node_from_end_of_list::remove_nth_from_end;
    use linkedlist::util::linked_list::{to_list, ListNode};

    use crate::stack::{
        simple_browser::Browser, stack_based_on_array::ArrayStack,
        stack_based_on_linked_list::LinkedListStack,
    };

    use crate::queue::{
        array_queue::ArrayQueue,
        circle_queue::CircleQueue,
        //  linked_list_queue::LinkedListQueue,
    };
    use crate::sort::{
        bubble_sort::bubble_sort, bucket_sort::bucket_sort, counting_sort::counting_sort,
        insertion_sort::insertion_sort, kth_largest::kth_largest, merge_sort::merge_sort,
        quick_sort::quick_sort, radix_sort::radix_sort, selection_sort::selection_sort,
        sort_string::sort_string,
    };

    use crate::binary_search::{
        binary_search, binary_search::binary_search, binary_search::binary_search_recursion,
        search_in_rotated_sorted_array::search, sqrtx::my_sqrt,
    };
    use crate::divide_and_conquer::merge_sort_count::merge_sort_count;

    #[test]
    fn array_rs() {
        let mut new_array = array_rs::NewArray::new(10);
        assert_eq!(new_array.insert(0, 3), true);
        assert_eq!(new_array.insert(1, 2), true);
        assert_eq!(new_array.insert(2, 8), true);
        assert_eq!(new_array.insert(0, 9), true);
        assert_eq!(new_array.insert(5, 7), false);
        assert_eq!(new_array.insert(4, 5), true);
        assert_eq!(new_array.find(3), 8);
        assert_eq!(new_array.find(12), -1);
        assert_eq!(new_array.remove(1), 3);
        assert_eq!(new_array.remove(9), -1);
    }

    #[test]
    fn linked_list() {
        println!("{:?}", has_cycle(to_list(vec![1, 2, 3, 4, 5])));

        println!(
            "{:?}",
            merge_two_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4]))
        );
        println!("{:?}", middle_node(to_list(vec![1, 3, 4])));
    }

    #[test]
    fn stack_based_on_array() {
        let mut stack = ArrayStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        stack.pop();
        println!("{:?}", stack.top());
    }

    #[test]
    fn stack_based_on_linked_list() {
        let mut stack = LinkedListStack::new();
        stack.push("https://www.baidu.com".to_string());
        stack.push("https://www.google.com".to_string());
        stack.pop();
        stack.push("https://twitter.com".to_string());
        stack.print_all();
    }

    #[test]
    fn stack_rs() {
        let mut browser = Browser::new();
        browser.open(&"http://www.baidu.com".to_string());
        browser.open(&"http://news.baidu.com/".to_string());
        browser.open(&"http://news.baidu.com/ent".to_string());
        browser.go_back();
        browser.go_back();
        browser.go_forward();
        browser.open(&"http://www.qq.com".to_string());
        browser.go_forward();
        browser.go_back();
        browser.go_forward();
        browser.go_back();
        browser.go_back();
        browser.go_back();
        browser.go_back();
        browser.check_current_page();
    }
    #[test]
    fn array_query() {
        let mut queue = ArrayQueue::new(3);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.dequeue();
        queue.dequeue();
        queue.enqueue(4);
        queue.dequeue();
        queue.print_all();
    }
    #[test]
    fn circle_queue() {
        let mut queue = CircleQueue::new(10);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.enqueue(2);
        queue.dequeue();
        queue.dequeue();
        queue.enqueue(4);
        queue.dequeue();
        queue.print_all();
    }
    #[test]
    fn linked_list_queue() {
        // let mut m = LinkedListQueue::new();
        // m.enqueue(4);
        // m.enqueue(4);
        // m.enqueue(4);
        // m.dequeue();
        // m.dequeue();
        // println!("{:?}", m);
    }
    #[test]
    fn bubble_sort_test() {
        let nums = vec![4, 5, 6, 1, 2, 3];
        println!("{:?}", bubble_sort(nums));
    }
    #[test]
    fn insert_on_sort_test() {
        let nums = vec![4, 5, 6, 1, 2, 3];
        println!("{:?}", insertion_sort(nums));
    }

    #[test]
    fn selection_sort_test() {
        let nums = vec![4, 5, 6, 1, 2, 3];
        println!("{:?}", selection_sort(nums));
    }
    #[test]
    fn kth_largest_test() {
        let nums = vec![8, 10, 2, 3, 6, 1, 5];
        println!("{:?}", kth_largest(nums, 3));
    }

    #[test]
    fn merge_sort_test() {
        let nums = vec![8, 10, 2, 3, 6, 1, 5];
        println!("{:?}", merge_sort(nums));
    }
    #[test]
    fn quick_sort_test() {
        let nums = vec![8, 10, 2, 3, 6, 1, 5];
        println!("{:?}", quick_sort(nums));
    }
    #[test]
    fn bucket_sort_test() {
        let nums = vec![2, 5, 3, 0, 2, 3, 0, 3];
        let m = bucket_sort(nums, 3);
        println!("{:?}", m);
    }
    #[test]
    fn counting_sort_test() {
        let nums = vec![2, 5, 3, 0, 2, 3, 0, 3];
        let m = counting_sort(nums);
        println!("{:?}", m);
    }
    #[test]
    fn radix_sort_test() {
        // let nums = vec![1, 10, 100, 1000, 98, 67, 3, 28, 67, 888, 777];
        let nums = vec![
            13111111111,
            13299999999,
            13311111111,
            13133333333,
            13922222222,
            13722222222,
        ];
        println!("{:?}", radix_sort(nums));
    }
    #[test]
    fn sort_string_test() {
        let s = "DaFBCA789".to_string();
        println!("{:?}", sort_string(s));
    }
    #[test]
    fn binary_search_test() {
        let nums5 = vec![8, 11, 19, 23, 27, 33, 45, 55, 67, 98];
        let nums6 = vec![8, 11, 19, 23, 27, 33, 45, 55, 67, 98];
        println!("{:?}", binary_search(nums5, 23));
        println!("{:?}", binary_search_recursion(nums6, 23));
        println!("{:?}", my_sqrt(8, 0.000001));
        let nums1 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
        let first_eq = binary_search::find_first_eq(nums1, 8);
        println!("{:?}", first_eq);

        let nums2 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
        let last_eq = binary_search::find_last_eq(nums2, 8);
        println!("{:?}", last_eq);

        let nums3 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
        let find_first_ge = binary_search::find_first_ge(nums3, 5);
        println!("{:?}", find_first_ge);

        let nums4 = vec![1, 3, 5, 6, 8, 8, 8, 11, 18];
        let find_last_le = binary_search::find_last_le(nums4, 17);
        println!("{:?}", find_last_le);
    }
    #[test]
    fn search_in_rotated_sorted_array() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let n = search(nums, 0);
        println!("{:?}", n);
    }
    #[test]
    fn hash_table() {
        let mut hash_table = MyHashTable::new();
        hash_table.insert("hello", "rust");
        println!("{:?}", hash_table);
        hash_table.insert("hi", "C++");
        println!("{:?}", hash_table);
        let m = hash_table.get("hello");
        println!("{:?}", m);
        let n = hash_table.remove("hi");
        println!("{:?}", n);
        println!("{:?}", hash_table);
    }
    #[test]
    fn heap_test() {
        let mut h = Heap::new(16);
        h.insert(33);
        h.insert(27);
        h.insert(21);
        h.insert(16);
        h.insert(13);
        h.insert(15);
        h.insert(9);
        h.insert(5);
        h.insert(6);
        h.insert(7);
        h.insert(8);
        h.insert(1);
        h.insert(2);
        h.insert(22);
        println!("{:?}", h);
        h.remove_max();
        println!("{:?}", h);
        h.remove_max();
        println!("{:?}", h);
        h.remove_max();
        println!("{:?}", h);

        let mut nums = vec![1, 4, 5, 7, 8, 13, 16, 19, 20];
        build_heap_down_up(&mut nums);
        println!("{:?}", nums);
        let mut nums1 = vec![1, 4, 5, 7, 8, 13, 16, 19, 20];
        build_heap_up_down(&mut nums1);
        println!("{:?}", nums1);

        let mut nums = vec![9, 6, 3, 1, 5];
        sort_heap::sort(&mut nums);
        println!("{:?}", nums);
    }
    #[test]
    fn get_median_test() {
        let mut nums = vec![12, 45, 30, 77, 5, 6, 7, 8];
        let m = get_median(&mut nums, 9);
        println!("{:?}", m); // 9
        let n = get_median(&mut nums, 20);
        println!("{:?}", n); // 12
        let h = get_median(&mut nums, 11);
        println!("{:?}", h); // 11
        let i = get_median(&mut nums, 10);
        println!("{:?}", i); // 11
    }
    #[test]
    fn get_top_k_test() {
        let mut nums = vec![4, 5, 7, 9, 10, 6, 11];
        let m = get_top_k::get_top_k(&mut nums, 3, 23);
        println!("{:?}", m);
    }
    #[test]
    fn merge_sorted_array_test() {
        let mut nums1 = vec![4, 5, 20, 90, 95, 100];
        let mut nums2 = vec![1, 6, 7, 8, 11, 23, 67, 89];
        let mut nums3 = vec![2, 5, 9, 30, 45];
        let new_nums = merge_sorted_array(&mut nums1, &mut nums2, &mut nums3);

        println!("{:?}", new_nums);
    }
    #[test]
    fn graph_test() {
        let mut graph = Graph::new(8);
        graph.add_edge(0, 1);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(4, 6);
        graph.add_edge(5, 7);
        graph.add_edge(6, 7);

        // Graph { v: 8, linked_vec: [[1, 3], [0, 2, 4], [1, 5], [0, 4], [1, 3, 5, 6], [2, 4, 7], [4, 7], [5, 6]] }
        println!("{:?}", graph);
        graph.bfs(0, 7);
        println!("bfs=============");
        graph.bfs(1, 3);
        println!("bfs=============");

        graph.dfs(0, 7);
        println!("dfs=============");
        graph.dfs(1, 3);
        println!("dfs=============");
    }

    #[test]
    fn bf_bk() {
        let primary = "thequickbrownfoxjumpsoverthelazydog";
        let pattern = "jump";
        let result = bf(primary, pattern);
        println!("{}", result); // 16

        let result2 = rk(primary, pattern);
        println!("{:?}", result2); // 16
    }
    #[test]
    fn bm_test() {
        let primary = "abcacabcbcabcabc";
        let pattern = "cabcab";
        let m = bm_search(primary, pattern);
        println!("{:?}", m);
    }
    #[test]
    fn kmp_test() {
        let primary1 = "abbaabbaaba";
        let pattern1 = "abbaaba";
        println!("{:?}", kmp_search(primary1, pattern1)); // 4

        let primary = "abc abcdab abcdabcdabde";
        let pattern = "bcdabd";
        println!("{:?}", kmp_search(primary, pattern)); // 16
    }
    #[test]
    fn trie_test() {
        let mut m = Trie::new();
        m.insert("hello");
        m.insert("she");
        println!("{:?}", m);
        let r = m.find("hello");
        println!("{}", r); // true
    }
    #[test]
    fn merge_sort_count() {
        let nums = vec![1, 5, 6, 2, 3, 4];
        let m = merge_sort_count(nums);
        println!("{:?}", m);
    }

    #[test]
    fn solve_bag_test() {
        // [(weight, value)...]
        let items = vec![(3, 5), (2, 2), (1, 4), (1, 2), (4, 10)];
        let m = solve_bag(items, 10);
        println!("{:?}", m); // {13: [1, 1, 1, 1, 0], 21: [1, 1, 1, 0, 1]}
    }
}
