mod sort;
mod graph;
mod search_tree;

// bin_search and fib_search are for ordered data
pub fn bin_search(numbers: &[i32], target: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high = numbers.len() - 1;

    while low <= high {
        // 在 rust 中，整数除法结果会自动取整
        let mid = (low + high) / 2;
        println!("low: {}, high: {}, mid: {}", low, high, mid);
        if numbers[mid] == target {
            return Some(mid);
        } else if numbers[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

pub fn fib_search(numbers: &[i32], target: i32) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn bin_search() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let target = 5;
        let result = super::bin_search(&numbers, target);
        assert_eq!(result, Some(4));
    }
}