/// ガウス・ルジャンドル積分（5点）
///
/// 5点のガウス求積法を用いて、区間 [a, b] で関数 f(x) を積分します。
/// 9次以下の多項式に対して厳密解を与えます。
fn gauss_legendre_5<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    // n=5 の積分点と重み
    // 対称性を利用して定義することも可能ですが、ここでは列挙します
    const NODES: [f64; 5] = [
        0.0,
        0.538_469_310_105_683_1,
        -0.538_469_310_105_683_1,
        0.906_179_845_938_664,
        -0.906_179_845_938_664,
    ];
    const WEIGHTS: [f64; 5] = [
        0.568_888_888_888_888_9,
        0.478_628_670_499_366_5,
        0.478_628_670_499_366_5,
        0.236_926_885_056_189_1,
        0.236_926_885_056_189_1,
    ];

    let mid = 0.5 * (a + b);
    let half_len = 0.5 * (b - a);
    let mut sum = 0.0;

    for i in 0..5 {
        let x = mid + half_len * NODES[i];
        sum += WEIGHTS[i] * f(x);
    }

    sum * half_len
}

fn main() {
    // 積分対象: f(x) = sin(x)
    // 区間: [0, PI]
    // 解析解: 2.0
    use std::f64::consts::PI;
    let f = |x: f64| x.sin();
    let a = 0.0;
    let b = PI;
    let exact = 2.0;

    let result = gauss_legendre_5(f, a, b);

    println!("Gaussian Quadrature (n=5)");
    println!("Result: {:.16}", result);
    println!("Error:  {:.2e}", (result - exact).abs());

    // 比較: シンプソン則 (n=10, 評価点数は11点)
    // ガウス積分(n=5)は評価点数が5点なので、それより多くの点数を使う条件で比較
    println!("\n(参考) Simpson's Rule (n=10)");
    // シンプソン則の実装は省略（前節参照）し、結果のオーダーのみ記述
    // 前節の結果から、n=10のとき誤差は約 6.8e-6 程度
}
