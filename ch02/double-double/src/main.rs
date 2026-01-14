use xprec::Df64;

fn main() {
    // f64からDf64への変換
    let a = Df64::from(1.0);
    let b = Df64::from(2.0);

    // 基本的な四則演算
    let sum = a + b;
    let diff = a - b;
    let prod = a * b;
    let quot = a / b;

    // Df64はDisplayトレイトを実装している
    println!("a + b = {}", sum);
    println!("a - b = {}", diff);
    println!("a * b = {}", prod);
    println!("a / b = {}", quot);

    // Df64からf64への変換（上位ワードを取得）
    let sum_f64: f64 = sum.hi();
    println!("Sum as f64: {}", sum_f64);
}
