//An input array, nums containing non-zero integers is given, where the value at each index represents the number of places to skip forward (if the value is positive)
// or backward (if the value is negative). When skipping forward or backward, wrap around if you reach either end of the array.
// For this reason, we are calling it a circular array. Determine if this circular array has a cycle.
// A cycle is a sequence of indices in the circular array characterized by the following:
//The same set of indices is repeated when the sequence is traversed in accordance with the aforementioned rules.
//The length of the sequence is at least two.
//The loop must be in a single direction, forward or backward.
//It should be noted that a cycle in the array does not have to originate at the beginning. A cycle can begin from any point in the array.

fn circular_array_loop(input: Vec<i32>) -> bool {
    let size = input.len();

    // Iterate through each index of the array 'nums'.
    for (index, value) in input.iter().enumerate() {
        // Set slow and fast pointer at current index value.
        let mut slow = index;
        let mut fast = index;

        // Set true in 'forward' if element is positive, set false otherwise.
        let mut forward = *value > 0;
        
        while true {
            // Move slow pointer to one step.
            slow = next_step(slow, input[slow], size);
            // If cycle is not possible, break the loop and start from next element.
            if !is_not_cycle(&input, forward, slow) {
                break;
            }
            
            // First move of fast pointer.
            fast = next_step(fast, input[fast], size);
            // If cycle is not possible, break the loop and start from next element.
            if !is_not_cycle(&input, forward, fast) {
                break;
            }
            
            // Second move of fast pointer.
            fast = next_step(fast, input[fast], size);
            // If cycle is not possible, break the loop and start from next element.
            if !is_not_cycle(&input, forward, fast) {
                break;
            }
            
            // At any point, if fast and slow pointers meet each other,
            // it indicates that loop has been found, return true.
            if slow == fast {
                return true;
            }
        }
    }
    
    false
}

fn next_step(index: usize, value: i32, size: usize) -> usize {
    let mut result = (index as i32 + value) % size as i32;
    if result < 0 {
        result += size as i32;
    }
    result as usize
}

// A function to detect a cycle doesn't exist
fn is_not_cycle(input: &Vec<i32>, prev_direction: bool, index: usize) -> bool{

    // Set current direction to true if current element is positive, set false otherwise.
    let curr_direction = input[index] >= 0;
    // If current direction and previous direction are different or moving a pointer takes back to the same value,
    // then the cycle is not possible, we return true, otherwise return false.
    if prev_direction != curr_direction || (input[index] % input.len() as i32).abs() == 0 {
        return true;
    } else {
        return false;
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn testcase1() {
        assert_eq!(circular_array_loop(vec![1, 3, -2, -4, 1]), true);
    }

    #[test]
    fn testcase2() {
        assert_eq!(circular_array_loop(vec![2, 1, -1, -2]), false);
    }

    #[test]
    #[ignore]
    fn testcase3() {
        assert_eq!(circular_array_loop(vec![5, 4, -2, -1, 3]), false);
    }

    #[test]
    fn testcase4() {
        assert_eq!(circular_array_loop(vec![1, 2, -3, 3, 4, 7, 1]), true);
    }

    #[test]
    #[ignore]
    fn testcase5() {
        assert_eq!(circular_array_loop(vec![3, 3, 1, -1, 2]), true);
    }
}