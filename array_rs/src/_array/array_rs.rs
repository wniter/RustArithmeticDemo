pub struct NewArray {
    array: Vec<i32>,
    count: i32,
}

impl NewArray {
    pub fn new(capacity: usize) -> NewArray {
        NewArray {
            array: Vec::with_capacity(capacity),
            count: 0,
        }
    }
    pub fn find(&self, index: usize) -> i32 {
        if index > self.count as usize {
            return -1;
        }
        self.array[index]
    }
    pub fn insert(&mut self, index: usize, value: i32) -> bool {
        let array_count = self.count as usize;
        if index > array_count || array_count == self.array.capacity() {
            return false;
        }
        if index == array_count {
            self.array.push(value);
        } else {
            let tmp_arr = self.array.clone();

            self.array = Vec::with_capacity(self.array.capacity());
            for i in 0..index {
                self.array.push(tmp_arr[i]);
            }
            self.array.push(value);

            for i in index..array_count {
                self.array.push(tmp_arr[i]);
            }
        }

        self.count += 1;
        true
    }
    pub fn remove(&mut self, index: usize) -> i32 {
        if index >= self.count as usize { return -1; }

        let result = self.array[index];
        let tmp_arr = self.array.clone();

        self.array = Vec::with_capacity(self.array.capacity());
        for i in 0..index {
            self.array.push(tmp_arr[i]);
        }

        for i in index+1..self.count as usize {
            self.array.push(tmp_arr[i]);
        }

        self.count -=1;
        result
    }
}
