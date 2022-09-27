// 二分查找
pub fn binary_search(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid = low + ((high - low) >> 1);
        if nums[mid] == value { return mid as i32; }

        if nums[mid] < value {
            low = mid + 1;
        } else {
            high = mid -1;
        }
    }
    -1
}

// 二分查找递归
pub fn binary_search_recursion(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    _recursion(&nums, 0, nums.len()-1, value)
}

fn _recursion(nums: &Vec<i32>, low: usize, high: usize, value: i32) -> i32 {
    if low > high { return -1; }

    let mid = low + ((high - low) >> 1);
    if nums[mid] == value { return mid as i32; }

    if nums[mid] < value {
        return _recursion(nums, mid+1, high, value);
    } else {
        return _recursion(nums, low, mid-1, value);
    }
}

// 查找第一个给定值的元素
pub fn find_first_eq(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + ((end - start) >> 1);
        if nums[mid] <= value {
            if mid == 0 || (nums[mid] == value && nums[mid-1] != value) { return mid as i32; }
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    -1
}

// 查找最后一个给定值的元素
pub fn find_last_eq(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + ((end - start) >> 1);
        if nums[mid] <= value {
            if mid == end || (nums[mid] == value && nums[mid+1] != value) { return mid as i32; }
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    -1
}

// 查找第一个大于等于给定值的元素
pub fn find_first_ge(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + ((end - start) >> 1);
        if nums[mid] < value {
            start = mid + 1;
        } else {
            if mid == end || (nums[mid] >= value && nums[mid-1] < value) { return mid as i32; }
            end = mid - 1;
        }
    }
    -1
}

// 查找最后一个小于等于给定值的元素
pub fn find_last_le(nums: Vec<i32>, value: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let mut start = 0;
    let mut end = nums.len() - 1;

    while start <= end {
        let mid = start + ((end - start) >> 1);
        if nums[mid] <= value {
            if mid == 0 || (nums[mid] <= value && nums[mid+1] > value) { return mid as i32; }
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    -1
}