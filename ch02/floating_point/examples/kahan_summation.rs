/// Kahan summationによる総和計算
fn kahan_sum(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0; // 累積誤差の補正項

    for &value in values {
        let y = value - compensation; // 補正を適用した値
        let t = sum + y; // 一時的な合計
        compensation = (t - sum) - y; // 丸め誤差を記録
        sum = t;
    }
    sum
}

fn main() {
    // 小さな値を大量に足し合わせる例
    let n = 1_000_000;
    let small_value = 0.1;
    let values: Vec<f64> = vec![small_value; n];

    // 単純な加算
    let naive_sum: f64 = values.iter().sum();

    // Kahan summation
    let kahan = kahan_sum(&values);

    // 理論値
    let expected = small_value * n as f64;

    println!("理論値:           {:.17}", expected);
    println!("単純な加算:       {:.17}", naive_sum);
    println!("Kahan summation: {:.17}", kahan);
    println!("\n単純な加算の誤差: {:.2e}", (naive_sum - expected).abs());
    println!("Kahanの誤差:      {:.2e}", (kahan - expected).abs());
}
