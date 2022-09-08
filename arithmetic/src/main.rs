fn main() {
    println!("Hello, world!");
    let sum_of_ints = sum(10, 20);
    println!("10 + 20 = {sum_of_ints}");
    let difference = difference(3, 5);
    println!("Difference of 3 and 5 = {difference}");
    let average_of_floats = average(12.5, 17.2); //good enough
    println!("Average of 12.5 and 17.2 = {average_of_floats}");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn difference(a: i32, b: i32) -> i32 {
    a - b
}
fn average(a: f32, b: f32) -> f32 {
    return (a + b) * 0.5;
}
