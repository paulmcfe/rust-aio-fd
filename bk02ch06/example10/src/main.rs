fn main() {
    let numbers: Vec<i32> = vec![4, 6, 3, 2, 1, 5];

    let how_many_nums = numbers.iter().count();
    let last_result = numbers.iter().last();
    let largest_result = numbers.iter().max();
    let smallest_result = numbers.iter().min();
    let product_of_nums: i32 = numbers.iter().product();
    let total_of_nums: i32 = numbers.iter().sum();
    println!("Elements: {}", how_many_nums);
    if let Some(last_num) = last_result {
        println!("Last element: {}", last_num);
    }
    if let Some(largest_num) = largest_result {
        println!("Largest element: {}", largest_num);
    }
    if let Some(smallest_num) = smallest_result {
        println!("Smallest element: {}", smallest_num);
    }
    println!("Product of elements: {}", product_of_nums);
    println!("Total of elements: {}", total_of_nums);
}
