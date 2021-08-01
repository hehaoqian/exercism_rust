pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let mut start = 0;
    let mut end = array.len();
    while start < end {
        let mid = (start + end) / 2;
        use std::cmp::Ordering::*;
        match array[mid].cmp(&key) {
            Equal => return Some(mid),
            Less => start = mid + 1,
            Greater => end = mid,
        }
    }
    None
}
