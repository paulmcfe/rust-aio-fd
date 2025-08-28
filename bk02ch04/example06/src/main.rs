fn sum_four(nums: [i32; 4]) -> i32 {
    nums[0] + nums[1] + nums[2] + nums[3]
}

fn main() {
    let array1: [i32; 4] = [1, 2, 3, 4];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    // This works fine
    println!("{}", sum_four(array1));

    // Error: expected an array with a size of 4, found one with a size of 5
    println!("{}", sum_four(array2));
}
