fn sum3(nums: [i32; 4]) -> i32 {
    nums[0] + nums[1] + nums[2] + nums[3]
}

fn main() {
    let array1: [i32; 4] = [1, 2, 3, 4];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", sum3(array1)); // works
    println!("{}", sum3(array2)); // error: expected [i32; 3], found [i32; 4]
}
