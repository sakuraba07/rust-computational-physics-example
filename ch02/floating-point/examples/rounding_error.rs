fn main() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let sum = a + b;
    let expected: f64 = 0.3;

    println!("0.1 + 0.2 = {}", sum);
    println!("期待値     = {}", expected);
    println!("両者は等しいか？ => {}", sum == expected);

    // 内部的な表現を確認
    println!("実際の値 (内部表現): {:.20}", sum);
}
