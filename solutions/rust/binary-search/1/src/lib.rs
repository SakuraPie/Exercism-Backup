pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() || key < array[0] {
        return None;
    }
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if array[mid] == key {
            return Some(mid);
        }
        if array[mid] > key {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}