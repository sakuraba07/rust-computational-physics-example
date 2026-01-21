fn main() {
    let nan = f64::NAN;
    println!("{}", nan + 1.0); // NaN
    println!("{}", nan * 0.0); // NaN
    println!("{}", nan.sin()); // NaN
    println!("{}", nan.max(0.0)); // 0（NaNではない！）
    println!("{}", nan.min(0.0)); // 0（NaNではない！）
}
