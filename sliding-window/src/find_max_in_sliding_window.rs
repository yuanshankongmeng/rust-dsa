//Given an integer list, nums, find the maximum values in all the contiguous subarrays (windows) of size w.
//If the window size is greater than the array size, we consider the entire array as a single window.

fn find_max_in_sliding_window(nums: Vec<i32>, w: usize) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }

    let adjusted_w = if w > nums.len() { nums.len() } else { w };

    let mut output = vec![0; nums.len() - adjusted_w + 1];
    let mut current_window: Vec<usize> = vec![];

    for index in 0..adjusted_w {
        clean_up(index, &mut current_window, &nums);

        current_window.push(index);
    }

    output[0] = nums[current_window[0]];

    for index in adjusted_w..nums.len() {
        clean_up(index, &mut current_window, &nums);

        if current_window.len() != 0 && current_window[0] <= index - adjusted_w {
            current_window.drain(0..1);
        }

        current_window.push(index);

        output[index - adjusted_w + 1] = nums[current_window[0]];
    }

    output
}

fn clean_up(index: usize, current_window: &mut Vec<usize>, nums: &Vec<i32>) {
    while current_window.len() != 0 && nums[index] > nums[current_window[current_window.len() - 1]]
    {
        current_window.pop();
    }
}

mod tests {
    use super::*;

    #[test]
    fn testcase1() {
        assert_eq!(
            find_max_in_sliding_window(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
            vec![3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn testcase2() {
        assert_eq!(
            find_max_in_sliding_window(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3], 4),
            vec![3, 3, 3, 3, 3, 3, 3]
        );
    }

    #[test]
    fn testcase3() {
        assert_eq!(
            find_max_in_sliding_window(
                vec![10, 6, 9, -3, 23, -1, 34, 56, 67, -1, -4, -8, -2, 9, 10, 34, 67],
                2
            ),
            vec![10, 9, 9, 23, 23, 34, 56, 67, 67, -1, -4, -2, 9, 10, 34, 67]
        );
    }

    #[test]
    fn testcase4() {
        assert_eq!(
            find_max_in_sliding_window(vec![4, 5, 6, 1, 2, 3], 1),
            vec![4, 5, 6, 1, 2, 3]
        );
    }

    #[test]
    fn testcase5() {
        assert_eq!(
            find_max_in_sliding_window(vec![9, 5, 3, 1, 6, 3], 2),
            vec![9, 5, 3, 6, 6]
        );
    }

    #[test]
    fn testcase6() {
        assert_eq!(find_max_in_sliding_window(vec![1, 2], 2), vec![2]);
    }
}
