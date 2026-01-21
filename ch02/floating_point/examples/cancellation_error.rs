fn main() {
    let a: f64 = 1.0;
    let b: f64 = 1.0e8; // 1億 (大きな値にすることで桁落ちが顕著になる)
    let c: f64 = 1.0;

    let d = (b * b - 4.0 * a * c).sqrt();

    // -b と d は非常に近い値になる (絶対値が近い)
    println!("-b = {}", -b);
    println!(" d = {}", d);

    // 桁落ちが発生する計算
    let x1_naive = (-b + d) / (2.0 * a);

    // もう一方の解
    let x2 = (-b - d) / (2.0 * a);

    println!("\nナイーブな計算:");
    println!("x1 = {:.17}", x1_naive);
    println!("x2 = {:.17}", x2);

    // 桁落ちを回避する工夫
    // 解と係数の関係 x1 * x2 = c/a を利用する
    let x1_stable = c / (a * x2);

    println!("\n安定な計算:");
    println!("x1 = {:.17}", x1_stable);

    // 相対誤差を計算
    let relative_error = (x1_naive - x1_stable).abs() / x1_stable.abs();
    println!(
        "\n相対誤差: {:.2e} ({:.1}%)",
        relative_error,
        relative_error * 100.0
    );
}
