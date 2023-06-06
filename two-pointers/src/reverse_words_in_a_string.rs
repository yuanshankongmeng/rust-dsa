// Given a sentence, reverse the order of its words without affecting the order of letters within a given word.
fn reverse_words(input: String) -> String {
    let mut words: Vec<&str> = input.split(" ").collect();
    
    if words.len() < 2 {
        return words.join(" ");
    }
    
    let mut min: usize = 0;
    let mut max: usize = words.len() - 1;
    
    while min < max {
        words.swap(min, max);
        min += 1;
        max -= 1;
    }
    
    words.join(" ")
}

mod tests {
    use super::*;
    
    #[test]
    fn test_case1() {
        assert_eq!(reverse_words("We love JavaScript".to_string()), "JavaScript love We");
    }
    
    #[test]
    fn test_case2() {
        assert_eq!(reverse_words("To be or not to be".to_string()), "be to not or be To");
    }

    #[test]
    fn test_case3() {
        assert_eq!(reverse_words("You are amazing".to_string()), "amazing are You");
    }

    #[test]
    fn test_case4() {
        assert_eq!(reverse_words("Hello     World".to_string()), "World     Hello");
    }

    #[test]
    fn test_case5() {
        assert_eq!(reverse_words("Hey".to_string()), "Hey");
    }
}