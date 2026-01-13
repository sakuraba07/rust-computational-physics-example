fn main() {
    let large_number: f64 = 1.0e16;
    let small_number: f64 = 1.0;

    let result = large_number + small_number - large_number;

    println!("(1.0e16 + 1.0) - 1.0e16 = {}", result);
    println!("期待値 = 1.0");
}
