//Statement
//Given an array of integers, nums, and an integer value, target,
//determine if there are any three integers in nums whose sum is equal to the target,
//that is, nums[i] + nums[j] + nums[k] == target. Return TRUE if three such integers exist in the array. Otherwise, return FALSE.

fn sum_of_three_values(mut input: Vec<i32>, target: i32) -> bool {
    // Sort the array
    input.sort();
    
    let mut index: usize = 0;
    
    while index < input.len() - 2 {
        let mut low: usize = index + 1;
        let mut high: usize = input.len() - 1;
        
        while low < high {
            let triple = input[index] + input[low] + input[high];
            if triple == target {
                return true;
            } else if triple < target {
                low += 1;
            } else {
                high -= 1;
            }
        }
        index += 1;
        
    }
    
    false
}

mod tests {
    use super::*;
    
    #[test]
    fn test_case1() {
        assert_eq!(sum_of_three_values(vec![1, -1, 0], -1), false);
    }

    #[test]
    fn test_case2() {
        assert_eq!(sum_of_three_values(vec![3, 7, 1, 2, 8, 4, 5], 10), true);
    }

    #[test]
    fn test_case3() {
        assert_eq!(sum_of_three_values(vec![3, 7, 1, 2, 8, 4, 5], 21), false);
    }

    #[test]
    fn test_case4() {
        assert_eq!(sum_of_three_values(vec![-1, 2, 1, -4, 5,-3], -8), true);
    }
    
    #[test]
    fn test_case5() {
        assert_eq!(sum_of_three_values(vec![-1, 2, 1, -4, 5, -3], 0), true);
    }
}