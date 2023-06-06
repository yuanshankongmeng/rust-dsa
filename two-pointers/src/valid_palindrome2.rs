// Write a function that takes a string as input and checks whether it can be a valid palindrome by removing at most one character from it.
fn valid_palindrome2(input: String) -> bool {
    let words: Vec<char> = input.chars().collect();

    let mut min = 0;
    let mut max = words.len() - 1;
    let mut removed_count = 0;

    while min < max {
        if words[min] == words[max] {
            min += 1;
            max -= 1;
        } else if words[min] == words[max - 1] {
            removed_count += 1;
            min += 1;
            max -= 2;
        } else if words[min + 1] == words[max] {
            removed_count += 1;
            min += 2;
            max -= 1;
        } else {
            return false;
        }
        if removed_count > 1 {
            return false;
        }
    }
    
    true
}

mod tests {
    use super::*;
    
    #[test]
    fn testcase1() {
        assert_eq!(valid_palindrome2("madame".to_string()), true);
    }

    #[test]
    fn testcase2() {
        assert_eq!(valid_palindrome2("dead".to_string()), true);
    }

    #[test]
    fn testcase3() {
        assert_eq!(valid_palindrome2("abca".to_string()), true);
    }

    #[test]
    fn testcase4() {
        assert_eq!(valid_palindrome2("tebbem".to_string()), false);
    }

    #[test]
    fn testcase5() {
        assert_eq!(valid_palindrome2("eeccccbebaeeabebccceea".to_string()), false);
    }
}