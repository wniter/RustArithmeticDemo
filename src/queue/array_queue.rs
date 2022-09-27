//create an array queue
#[derive(Debug)]
pub struct ArrayQueue {
    queue: Vec<i32>,
    head: i32,
    tail: i32,
}

impl ArrayQueue {
    pub fn new(n: usize) -> Self {
        ArrayQueue {
            queue: Vec::with_capacity(n),
            head: 0,
            tail: 0,
        }
    }
    pub fn enqueue(&mut self, num: i32) -> bool {
        //gain the queue of capacity
        let c = self.queue.capacity() as i32;
        // queue is full
        if self.head == 0 && self.tail == c {
            return false;
        }

        if self.tail == c {
            for i in 0..(self.tail -self.head) as usize {
                self.queue[i] = self.queue[self.head as usize + i];
            }
            self.tail -= self.head;
            self.head = 0;
            self.queue[self.tail as usize] = num;
        } else {
            self.queue.push(num);
        }
        self.tail += 1;
        return true;
    }
    pub fn dequeue(&mut self) -> i32 {
        if self.head == self.tail { return -1; }

        let shift = self.queue[self.head as usize];
        self.head += 1;
        shift
    }
    pub fn print_all(&mut self) {
        let mut s= String::from("");
         for i in self.head..self.tail {
            s.push(self.queue[i as usize] as u8 as char);
            s.push_str("->");
        }
        println!("{:?}",s);
    }
}
