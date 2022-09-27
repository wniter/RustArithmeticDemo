#[derive(Debug)]
pub struct ArrayStack {
    data: Vec<i32>,
    top: i32,
}

impl ArrayStack {
    pub fn new() -> Self {
        ArrayStack {
            data: Vec::with_capacity(32),
            top: -1,
        }
    }

    pub fn push(&mut self, x: i32) {
        self.top += 1;
        if self.top == self.data.capacity() as i32 {
            let tmp_arr = self.data.clone();
            self.data = Vec::with_capacity(self.data.capacity() * 2);
            for d in tmp_arr.into_iter() {
                self.data.push(d);
            }
        }
        self.data.push(x);
    }

    pub fn pop(&mut self) {
        if self.is_empty() {
            return;
        }
        self.top -= 1;
        self.data.pop();
    }

    pub fn top(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        *self.data.last().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        if self.top < 0 {
            true
        } else {
            false
        }
    }
}
