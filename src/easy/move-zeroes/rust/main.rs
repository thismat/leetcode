fn main() {
    let mut input1: Vec<i32> = vec![0, 1, 0, 3, 12];

    println!("Running with Input1:, {:?}", input1);
    move_zeroes(&mut input1);
    println!("Input1 Results:, {:?}", input1);

    let mut input2: Vec<i32> = vec![1, 0, 1];
    println!("Running with Input2:, {:?}", input2);
    move_zeroes(&mut input2);
    println!("Input2 Results:, {:?}", input2);

    let mut input3: Vec<i32> = vec![1, 0];
    println!("Running with Input3:, {:?}", input3);
    move_zeroes(&mut input3);
    println!("Input3 Results:, {:?}", input3);

    let mut input4: Vec<i32> = vec![0, 1];
    println!("Running with Input4:, {:?}", input4);
    move_zeroes(&mut input4);
    println!("Input4 Results:, {:?}", input4);

    let mut input5: Vec<i32> = vec![2, 1];
    println!("Running with Input5:, {:?}", input5);
    move_zeroes(&mut input5);
    println!("Input5 Results:, {:?}", input5);

    let mut input6: Vec<i32> = vec![4, 2, 4, 0, 0, 3, 0, 6, 1, 0];
    println!("Running with Input6:, {:?}", input6);
    move_zeroes(&mut input6);
    println!("Input6 Results:, {:?}", input6);

    let mut input7: Vec<i32> = vec![4, 2, 4, 0, 0, 3, 0, 7, 1, 0];
    println!("Running Cooler Version with Input7:, {:?}", input7);
    move_zeroes_2(&mut input7);
    println!("Input7 Results:, {:?}", input7);
}

// I had to look at the solution, I knew roughly what it was but I was too slow
pub fn move_zeroes(nums: &mut Vec<i32>) {
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

// This is a cool example I found from LeetCode after submission.
pub fn move_zeroes_2(nums: &mut Vec<i32>) {
    let mut write_idx = 0;

    for read_idx in 0..nums.len() {
        if nums[read_idx] != 0 {
            nums.swap(write_idx, read_idx);
            write_idx += 1;
        }
    }
}
