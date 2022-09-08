fn main() {
    println!("Hello, world!");
    let sum_of_ints = sum_func(10, 20);
    println!("10 + 20 = {sum_of_ints}");
    let average_of_floats = average_func(12.5, 17.2); //good enough
    println!("Average of 12.5 and 17.2 = {average_of_floats}");
}

fn sum_func(a: i32, b: i32) -> i32 {
    a + b
}

fn average_func(a: f32, b: f32) -> f32 {
    return (a + b) * 0.5;
}
