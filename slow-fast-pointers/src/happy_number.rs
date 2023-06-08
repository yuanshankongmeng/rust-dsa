fn is_happy_number(input: i64) -> bool {
    let mut slow_pointer = input;
    let mut fast_pointer = sum_of_square_digits(input);
    
    while fast_pointer != 1 && slow_pointer != fast_pointer {
        slow_pointer = sum_of_square_digits(slow_pointer);
        fast_pointer = sum_of_square_digits(sum_of_square_digits(fast_pointer));
    }
    
    if fast_pointer == 1 {
        return true;
    }
    false
}

fn sum_of_square_digits(input: i64) -> i64 {
    input.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .map(|d| d * d)
        .fold(0, |acc, d| acc + d as i64)
}

mod tests {
    use super::*;
    
    #[test]
    fn testcase1() {
        assert_eq!(is_happy_number(2147483646), false);
    }

    #[test]
    fn testcase2() {
        assert_eq!(is_happy_number(1), true);
    }

    #[test]
    fn testcase3() {
        assert_eq!(is_happy_number(19), true);
    }

    #[test]
    fn testcase4() {
        assert_eq!(is_happy_number(8), false);
    }

    #[test]
    fn testcase5() {
        assert_eq!(is_happy_number(7), true);
    }
}