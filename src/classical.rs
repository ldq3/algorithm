static NUMBERS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

fn convert(n: usize, target_base: usize) -> String {
    let mut stack: Vec<usize> = Vec::new();
    let mut n = n;

    while n > 0 {
        let remainder = n % target_base;
        stack.push(remainder);
        n /= target_base;
    }

    let mut result = String::new();
    while let Some(digit) = stack.pop() {
        result.push_str(&NUMBERS[digit].to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        assert_eq!(super::convert(10, 2), "1010");

        assert_eq!(super::convert(10, 16), "a");

        assert_eq!(super::convert(10, 8), "12");
    }
}