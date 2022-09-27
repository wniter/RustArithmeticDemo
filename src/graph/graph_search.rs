use std::collections::LinkedList;
use std::collections::VecDeque;

// 无向图
#[derive(Debug)]
pub struct Graph {
    v: i32,
    linked_vec: Vec<LinkedList<i32>>, // 邻接表
}

impl Graph {
    pub fn new(v: i32) -> Self {
        Graph {
            v: v,
            linked_vec: vec![LinkedList::new(); v as usize],
        }
    }

    // 无向图的每条边存两次
    pub fn add_edge(&mut self, s: i32, t: i32) {
        self.linked_vec[s as usize].push_back(t);
        self.linked_vec[t as usize].push_back(s);
    }

    pub fn bfs(&self, s: i32, t: i32) {
        if s == t {
            return;
        }

        let mut prev = vec![-1; self.v as usize];
        let mut visited = vec![false; self.v as usize];
        let mut queue = VecDeque::new();

        visited[s as usize] = true;
        queue.push_back(s);

        while !queue.is_empty() {
            let w = queue.pop_front().unwrap();
            for item in self.linked_vec[w as usize].iter() {
                if visited[*item as usize] {
                    continue;
                }
                prev[*item as usize] = w;
                if *item == t {
                    self.draw(&prev, s, t);
                    return;
                }
                visited[*item as usize] = true;
                queue.push_back(*item);
            }
        }
    }

    pub fn dfs(&self, s: i32, t: i32) {
        let mut found = false;
        let mut prev = vec![-1; self.v as usize];
        let mut visited = vec![false; self.v as usize];

        self.recur_dfs(s, t, &mut visited, &mut prev, &mut found);
        self.draw(&prev, s, t);
    }

    pub fn recur_dfs(
        &self,
        s: i32,
        t: i32,
        visited: &mut Vec<bool>,
        prev: &mut Vec<i32>,
        found: &mut bool,
    ) {
        if *found == true {
            return;
        }
        visited[s as usize] = true;
        if s == t {
            *found = true;
            return;
        }
        for item in self.linked_vec[s as usize].iter() {
            if visited[*item as usize] {
                continue;
            }
            prev[*item as usize] = s;
            self.recur_dfs(*item, t, visited, prev, found);
        }
    }

    // 递归打印路径
    pub fn draw(&self, prev: &Vec<i32>, s: i32, t: i32) {
        if prev[t as usize] != -1 && s != t {
            self.draw(prev, s, prev[t as usize]);
        }

        println!("{} ->", t);
    }
}
