/**
 *
 * This is the optimal solution using a slow/fast two pointer solution
 *
 */
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut write_idx = 0;

    for read_idx in 0..nums.len() {
        if nums[read_idx] != 0 {
            nums.swap(write_idx, read_idx);
            write_idx += 1;
        }
    }
}

/**
 *
 *  This is my original solution, not super great but it worked.
 *  I am pretty sure I peeked for solutions after getting 90% there.
 *
 */
pub fn move_zeroes_my_solution(nums: &mut Vec<i32>) {
    if nums.len() == 1 {
        return;
    }

    let mut slow: usize = 0;
    for fast in 0..nums.len() {
        if nums[fast] != 0 && nums[slow] == 0 {
            let swap_to_fast = nums[slow];
            nums[slow] = nums[fast];
            nums[fast] = swap_to_fast;
        }

        if nums[slow] != 0 {
            slow += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input: Vec<i32> = vec![0, 1, 0, 3, 12];
        let expected: Vec<i32> = vec![1, 3, 12, 0, 0];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }

    #[test]
    fn test_2() {
        let mut input: Vec<i32> = vec![1, 0, 1];
        let expected: Vec<i32> = vec![1, 1, 0];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }

    #[test]
    fn test_3() {
        let mut input: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![1, 0];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }

    #[test]
    fn test_4() {
        let mut input: Vec<i32> = vec![2, 1];
        let expected: Vec<i32> = vec![2, 1];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }

    #[test]
    fn test_5() {
        let mut input: Vec<i32> = vec![4, 2, 4, 0, 0, 3, 0, 6, 1, 0];
        let expected: Vec<i32> = vec![4, 2, 4, 3, 6, 1, 0, 0, 0, 0];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }

    #[test]
    fn test_6() {
        let mut input: Vec<i32> = vec![4, 2, 4, 0, 0, 3, 0, 7, 1, 0];
        let expected: Vec<i32> = vec![4, 2, 4, 3, 7, 1, 0, 0, 0, 0];

        move_zeroes(&mut input);
        assert_eq!(input, expected, "Input: {:?} Answer: {:?}", input, expected);
    }
}

