#![warn(clippy::all, clippy::pedantic)]

use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    let result = bin_search(&arr, 24);
    let result_hash = hsmp_search(&arr, 42);

    match result {
        Some((found_value, found_index)) => {
            println!("Found item - {found_value}; found index - {found_index}");
        }
        None => println!("Not found!"),
    };

    match result_hash {
        Some((found_value, found_index)) => {
            println!("Found item - {found_value}; found index - {found_index}");
        }
        None => println!("Not found!"),
    };
}

fn bin_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    let mut low: usize = 0;
    let mut up: usize = arr.len() - 1;

    while low <= up {
        let mid = (low + up) / 2;
        let mid_value: i32 = arr[mid];

        match mid_value.cmp(&target) {
            Ordering::Equal => return Some((arr[mid], mid)),
            Ordering::Greater => up = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }
    None
}

fn create_hashmap(arr: &[i32]) -> HashMap<&i32, usize> {
    let mut map = HashMap::new();

    for (index, item) in arr.iter().enumerate() {
        map.insert(item, index);
    }

    map
}

fn hsmp_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    let hash = create_hashmap(arr);

    if let Some(&index) = hash.get(&target) {
        Some((arr[index], index))
    } else {
        None
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const ARR: [i32; 10] = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1234);
        assert!(result.is_none());
    }

    #[test]
    fn element_found_hsmp() {
        assert_eq!((-1, 0), hsmp_search(&ARR, -1).unwrap());
    }

    #[test]
    fn element_not_found_hsmp() {
        let result = hsmp_search(&ARR, 1234);
        assert!(result.is_none());
    }
}
