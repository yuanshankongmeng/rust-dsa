//Given an unsorted array of positive numbers, nums, such that the values lie in the range [1, n] inclusive,
//and that there are n + 1
//numbers in the array, find and return the duplicate number present in nums. There is only one repeated number in nums
// Note: You cannot modify the given array nums. You have to solve the problem using only constant extra space.

fn find_duplicate_number(input: Vec<usize>) -> usize {
    let mut slow = input[0];
    let mut fast = input[0];
    
    while true {
        slow = input[slow];
        fast = input[input[fast]];
        
        if slow == fast {
            break;
        }
    }
    
    slow = input[0];
    while slow != fast {
        slow = input[slow];
        fast = input[fast];
    }
    
    fast
}

mod tests {
    use super::*;
    
    #[test]
    fn testcase1() {
        assert_eq!(find_duplicate_number(vec![3, 4, 4, 4, 2]), 4);
    }
    
    #[test]
    fn testcase2() {
        assert_eq!(find_duplicate_number(vec![1, 1]), 1);
    }

    #[test]
    fn testcase3() {
        assert_eq!(find_duplicate_number(vec![1, 3, 4, 2, 2]), 2);
    }
    
    #[test]
    fn testcase4() {
        assert_eq!(find_duplicate_number(vec![1, 3, 6, 2, 7, 3, 5, 4]), 3);
    }

    #[test]
    fn testcase5() {
        assert_eq!(find_duplicate_number(vec![1, 2, 2]), 2);
    }
}