fn valid_palindrome(input: String) -> bool {
    // Convert string to a vec of chars
    let vec: Vec<char> = input.chars().collect();
    
    // if the length of vec is less than 2 then it is not a valid palindrome
    if vec.len() < 2 {
        return false;
    }
    
    // Use two pointers strategy to determine if it is a valid palindrome
    let mut left: usize = 0;
    let mut right = input.len() - 1;
    
    while left < right {
        if vec[left] != vec[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    
    true
}

mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        assert_eq!(valid_palindrome("".to_string()), false);
    }

    #[test]
    fn test_one_character() {
        assert_eq!(valid_palindrome("a".to_string()), false);
    }
    
    #[test]
    fn test_valid_case1() {
        assert_eq!(valid_palindrome("kayak".to_string()), true);
    }

    #[test]
    fn test_invalid_case1() {
        assert_eq!(valid_palindrome("hello".to_string()), false);
    }

    #[test]
    fn test_invalid_case2() {
        assert_eq!(valid_palindrome("RACEACAR".to_string()), false);
    }

    #[test]
    fn test_invalid_case3() {
        assert_eq!(valid_palindrome("ABCDABCD".to_string()), false);
    }
    
    #[test]
    fn test_valid_case2() {
        assert_eq!(valid_palindrome("DCBAABCD".to_string()), true);
    }
}