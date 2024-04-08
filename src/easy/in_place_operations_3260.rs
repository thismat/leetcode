/**
 *
 * My solution TBD :D
 *
 */
pub fn sort_array_by_parity_my_solution(nums: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![0; nums.len()];

    let mut even_insert: usize = 0;
    let mut odd_insert: usize = nums.len() - 1;

    for (_, num) in nums.iter().enumerate() {
        if num % 2 == 0 {
            // It is even
            results[even_insert] = *num;
            even_insert += 1;
        } else {
            // It is odd
            results[odd_insert] = *num;
            odd_insert -= 1;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input: Vec<i32> = vec![3, 1, 2, 4];

        let results: Vec<i32> = sort_array_by_parity_my_solution(input);

        for (index, num) in results.iter().enumerate() {
            if index < results.len() / 2 {
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

        println!("Test");
    }

    #[test]
    fn test_2() {
        let input: Vec<i32> = vec![0];

        let results: Vec<i32> = sort_array_by_parity_my_solution(input);

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

