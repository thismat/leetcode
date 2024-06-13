/**
 *
 * My solution, similar to other sliding window problems
 *
 */
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write_idx = 0;
    for read_idx in 0..nums.len() {
        if nums[read_idx] != val {
            nums.swap(write_idx, read_idx);
            write_idx += 1;
        }
    }
    write_idx as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input: Vec<i32> = vec![0, 1, 0, 3, 12];

        let results = remove_element(&mut input, 0);
        for i in 0..results {
            assert_ne!(input[i as usize], 0);
        }
    }

    #[test]
    fn test_2() {
        let mut input: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let results = remove_element(&mut input, 2);
        for i in 0..results {
            assert_ne!(input[i as usize], 2);
        }
    }

    #[test]
    fn test_3() {
        let mut input: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let results = remove_element(&mut input, 2);
        for i in 0..results {
            assert_ne!(input[i as usize], 2);
        }
    }
}

