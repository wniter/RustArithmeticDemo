/**
 * 在同一个cargo文件中，创建目录，先添加mod，在添加use
 * 一个文件名也算一个mod
 */
pub mod _array;
use  _array::array_rs;
fn main() {
    // println!("Hello, world!");
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
