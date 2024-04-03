/**
 *
 * My solution TBD :D
 *
 */
pub fn sort_array_by_parity_my_solution(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![0; nums.len()];

    // TODO: I can do this instead by subtracting the length with the current index
    for (index, num) in nums.iter().enumerate() {
        if num % 2 == 0 {
            // It is even
            results[index] = *num;
        } else {
            // It is odd
            results[nums.len() - 1 - index] = *num;
        }
    }

    results
}

/**
 *
 *  Optimal Solution
 *
 */
pub fn sort_array_by_parity_optimal(nums: Vec<i32>) -> Vec<i32> {
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: Vec<i32> = vec![3, 1, 2, 4];

        let results: Vec<i32> = sort_array_by_parity_my_solution(input);

        for (index, num) in results.iter().enumerate() {
            if index <= results.len() {
                assert_eq!(
                    num % 2,
                    0,
                    "Number: {}, is not even at index: {}, in array length: {} - Array: {:?}",
                    num,
                    index,
                    results.len(),
                    results
                );
            } else {
                assert_eq!(
                    num % 2,
                    1,
                    "Number: {}, is not odd at index: {}, in array length: {}",
                    num,
                    index,
                    results.len()
                );
            }
        }
    }

    #[test]
    fn test_2() {
        let input: Vec<i32> = vec![0];

        let results: Vec<i32> = sort_array_by_parity_optimal(input);

        for (index, num) in results.iter().enumerate() {
            if index <= results.len() {
                assert_eq!(
                    num % 2,
                    0,
                    "Number: {}, is not even at index: {}, in array length: {}",
                    num,
                    index,
                    results.len()
                );
            } else {
                assert_eq!(
                    num % 2,
                    1,
                    "Number: {}, is not odd at index: {}, in array length: {}",
                    num,
                    index,
                    results.len()
                );
            }
        }
    }
}

