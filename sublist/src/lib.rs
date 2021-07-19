#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist_strict<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    if short.len() >= long.len() {
        return false;
    }
    (0..=(long.len() - short.len())).any(|i| &long[i..i + short.len()] == short)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use std::cmp::Ordering;
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal => {
            if _first_list == _second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if is_sublist_strict(_first_list, _second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if is_sublist_strict(_second_list, _first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
