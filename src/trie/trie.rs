// [leetcode 208](https://leetcode.com/problems/implement-trie-prefix-tree/)

#[derive(Default, Debug)]
pub struct Trie {
    is_ending: bool,
    nodes: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr = self;
        for i in word.chars().map(|c| (c as usize - 'a' as usize) as usize) {
            curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_ending = true;
    }

    pub fn find(&self, word: &str) -> bool {
        let mut curr = self;
        for i in word.chars().map(|c| (c as usize - 'a' as usize) as usize) {
            match curr.nodes[i].as_ref() {
                Some(node) => {
                    curr = node;
                }
                None => {
                    return false;
                }
            }
        }
        curr.is_ending
    }
}
