fn adaptive_simpson_recursive<F>(
    f: &F,
    a: f64,
    b: f64,
    eps: f64,
    s: f64,
    fa: f64,
    fb: f64,
    fm: f64,
    depth: usize,
) -> f64
where
    F: Fn(f64) -> f64,
{
    let m = 0.5 * (a + b);
    let h = b - a;

    // 左右の分点
    let lm = 0.5 * (a + m);
    let rm = 0.5 * (m + b);

    // 関数評価
    let flm = f(lm);
    let frm = f(rm);

    // 左右の小区間でのシンプソン値
    // S(a, m)
    let s_left = (h * 0.5) / 6.0 * (fa + 4.0 * flm + fm);
    // S(m, b)
    let s_right = (h * 0.5) / 6.0 * (fm + 4.0 * frm + fb);

    let s2 = s_left + s_right;

    // 誤差推定
    let error = (s2 - s).abs() / 15.0;

    // 停止条件: 誤差が許容値以下 または 再帰が深すぎる場合
    if error <= eps || depth == 0 {
        // 誤差補正を含めた値を返す (Richardson extrapolation)
        s2 + (s2 - s) / 15.0
    } else {
        // 再帰的に分割
        // 許容誤差も分割に応じてスケールさせる（半分にする）のが一般的
        let eps_half = eps * 0.5;
        adaptive_simpson_recursive(f, a, m, eps_half, s_left, fa, fm, flm, depth - 1)
            + adaptive_simpson_recursive(f, m, b, eps_half, s_right, fm, fb, frm, depth - 1)
    }
}

/// 適応型シンプソン積分のエントリポイント
fn adaptive_simpson<F>(f: F, a: f64, b: f64, tol: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let m = 0.5 * (a + b);
    let h = b - a;
    let fa = f(a);
    let fb = f(b);
    let fm = f(m);

    // 全区間でのシンプソン値
    let s = h / 6.0 * (fa + 4.0 * fm + fb);

    // 最大再帰深さ（スタックオーバーフロー防止）
    let max_depth = 50;

    adaptive_simpson_recursive(&f, a, b, tol, s, fa, fb, fm, max_depth)
}

fn main() {
    use std::f64::consts::PI;

    // 積分対象: 鋭いガウスピークを持つ関数
    // x = 0.5 にピーク、幅が狭い
    let f = |x: f64| (-100.0 * (x - 0.5).powi(2)).exp();

    let a = 0.0;
    let b = 1.0;
    let tolerance = 1e-8;

    // 適応型積分
    let result = adaptive_simpson(f, a, b, tolerance);

    // 比較用：固定刻みシンプソン則 (N=100)
    let n_fixed = 100;
    let h_fixed = (b - a) / n_fixed as f64;
    let mut s_fixed = f(a) + f(b);
    for i in 1..n_fixed {
        let x = a + i as f64 * h_fixed;
        s_fixed += if i % 2 == 0 { 2.0 } else { 4.0 } * f(x);
    }
    s_fixed *= h_fixed / 3.0;

    // 解析解に近い値（高精度計算の結果）
    // int_0^1 exp(-100(x-0.5)^2) dx approx sqrt(pi)/10 * erf(5)
    // erf(5) approx 0.999999999998... approx 1.0
    let exact = (PI / 100.0).sqrt() * libm::erf(5.0);

    println!("Target: Gaussian peak at x=0.5");
    println!("Exact:            {:.12}", exact);
    println!(
        "Adaptive Simpson: {:.12} (Error: {:.2e})",
        result,
        (result - exact).abs()
    );
    println!(
        "Fixed Simpson:    {:.12} (Error: {:.2e})",
        s_fixed,
        (s_fixed - exact).abs()
    );
}
