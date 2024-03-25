// My silly attempt at my first answer. I don't know what I'm doing and it shows :D
pub fn remove_duplicates_my_solution(nums: &mut Vec<i32>) -> i32 {
    let mut results_array: Vec<i32> = Vec::new();
    let mut duplicates: i32 = 0;

    let mut pi: Option<usize> = None;

    for (index, num) in nums.iter().enumerate() {
        let pidx = match pi {
            Some(_p) => index - 1,
            None => index,
        };

        if nums[pidx] == *num && index > 0 {
            duplicates += 1;
        } else {
            results_array.push(*num);
        }

        pi = Some(index)
    }

    let unique = results_array.len();

    for _n in 0..duplicates {
        results_array.push(-1);
    }

    for n in 1..results_array.len() {
        nums[n] = results_array[n];
    }

    unique as i32
}

// This was not my answer, this was the most optimal answer I saw on LeetCodes Result Screen
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // Unique values will be at least 1
    let mut j = 1;

    // Start at 1 so you can look back at the first pair
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input: Vec<i32> = vec![1, 1, 2];

        let expected_vec: Vec<i32> = vec![1, 2, 1];
        let expected_unique: i32 = 2;

        let answer = remove_duplicates(&mut input);

        assert_eq!(
            input.len(),
            expected_vec.len(),
            "input and expected are the same length. expected: {}, input: {}",
            expected_vec.len(),
            input.len()
        );

        assert_eq!(
            answer, expected_unique,
            "expected: {}, got: {}",
            expected_unique, answer
        );

        for num in 0..answer {
            let index: usize = num as usize;

            assert_eq!(
                input[index], expected_vec[index],
                "index: {}, expected: {}, got: {}",
                index, input[index], expected_vec[index]
            );
        }
    }

    #[test]
    fn test_2() {
        let mut input: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let expected_vec: Vec<i32> = vec![0, 1, 2, 3, 4, 0, 0, 0, 0, 0];
        let expected_unique: i32 = 5;

        let answer = remove_duplicates(&mut input);

        assert_eq!(
            input.len(),
            expected_vec.len(),
            "input and expected are the same length. expected: {}, input: {}",
            expected_vec.len(),
            input.len()
        );

        assert_eq!(
            answer, expected_unique,
            "Number of results expected: {}, got: {}",
            expected_unique, answer
        );

        for num in 0..answer {
            let index: usize = num as usize;

            assert_eq!(
                input[index], expected_vec[index],
                "index: {}, expected: {}, got: {}",
                index, input[index], expected_vec[index]
            );
        }
    }
}

fn run() {
    let mut input2: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    let results2 = remove_duplicates(&mut input2);
    println!("results 2: {results2}");
    println!("results 2: {input2:?}");
}

