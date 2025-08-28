fn zero_out(nums: &mut [i32]) {
    for x in nums {
        // We need to dereference here!
        *x = 0;
    }
}

fn main() {
    // Declare a mutable array
    let mut arr = [12, 22, 31, 69, 82, 91, 5, 44, 50, 79];

    // Send a mutable slice to the function
    zero_out(&mut arr[..5]);

    // Check out the values in arr now
    println!("{:?}", arr);

        let mut hue = 30;
    let complement = &mut hue;
    *complement += 180;
    println!("{hue}"); // Output: 210
}
