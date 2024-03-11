use std::iter::once;

fn main() {
    println!("Remove Duplicates In Place");
    let mut input1: Vec<i32> = vec![1, 1, 2];

    let results1 = remove_duplicates(&mut input1);
    println!("results 1: {results1}");
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut results_array: Vec<&i32> = Vec::new();
    let mut duplicates: i32 = 0;

    for (index, num) in nums.iter().enumerate() {
        let previous_index: i32 = index - 1;
        let mut previous_number: i32 = -1;

        if previous_index >= 0 {
            previous_number = nums[previous_index];
        }

        if *num != previous_number {
            results_array.push(num);
        } else {
            duplicates += 1;
        }
    }

    results_array.len() as i32
}
