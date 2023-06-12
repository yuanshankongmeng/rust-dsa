// Given a string, s, that represents a DNA subsequence, and a number k return all the contiguous subsequences (substrings) of length
// that occur more than once in the string. The order of the returned subsequences does not matter. If no repeated substring is found, the function should return an empty set.

use std::collections::hash_map::*;

fn repeated_dns_sequences(s: String, k: usize) -> Vec<String> {
    let mut target = vec![];
    let mut cache = HashMap::new();

    let mut index = 0;
    let arr: Vec<char> = s.chars().collect();
    while index + k < arr.len() as usize {
        let sub = arr[index..index + k].iter().collect::<String>();
        if !cache.contains_key(&sub) {
            cache.insert(sub, 0);
        } else {
            target.push(sub);
        }
        index += 1;
    }

    target
}

mod tests {
    use super::*;

    #[test]
    fn testcase1() {
        assert_eq!(
            repeated_dns_sequences("AAAAACCCCCAAAAACCCCCC".to_string(), 8),
            vec!["AAAAACCC", "AAAACCCC", "AAACCCCC"]
        );
    }

    #[test]
    fn testcase2() {
        assert_eq!(
            repeated_dns_sequences("GGGGGGGGGGGGGGGGGGGGGGGGG".to_string(), 12).sort(),
            vec!["GGGGGGGGGGGG"].sort()
        );
    }

    #[test]
    fn testcase3() {
        assert_eq!(
            repeated_dns_sequences("TTTTTCCCCCCCTTTTTTCCCCCCCTTTTTTT".to_string(), 10).sort(),
            vec![
                "CCCCCCCTTT",
                "CCCCCCTTTT",
                "CCCCCTTTTT",
                "CCCCTTTTTT",
                "TCCCCCCCTT",
                "TTCCCCCCCT",
                "TTTCCCCCCC",
                "TTTTCCCCCC",
                "TTTTTCCCCC"
            ]
            .sort()
        );
    }

    #[test]
    fn testcase4() {
        assert_eq!(
            repeated_dns_sequences("AAAAAACCCCCCCAAAAAAAACCCCCCCTG".to_string(), 10).sort(),
            vec!["AAAAAACCCC", "AAAAACCCCC", "AAAACCCCCC", "AAACCCCCCC"].sort()
        );
    }

    #[test]
    fn testcase5() {
        assert_eq!(
            repeated_dns_sequences("ATATATATATATATAT".to_string(), 6).sort(),
            vec!["ATATAT", "TATATA"].sort()
        );
    }
}
