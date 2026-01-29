fn main() {
    // 目的関数とその勾配
    let f = |x: &[f64]| x[0].powi(2) + x[1].powi(2);
    let grad = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];

    let mut x = vec![2.0, 1.0]; // 初期値
    let alpha = 0.4; // 学習率
    let max_iter = 100;

    for i in 0..max_iter {
        let current_val = f(&x);
        let g = grad(&x);

        // 勾配の大きさが十分小さくなったら終了
        let g_norm: f64 = g.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        if g_norm < 1e-6 {
            println!(
                "\n収束しました: x={:?}, f(x)={:.6} (反復: {})",
                x, current_val, i
            );
            return;
        }

        // 更新 x = x - alpha * grad
        for j in 0..2 {
            x[j] -= alpha * g[j];
        }

        println!("iter {}: x={:?}, f(x)={:.6}", i, x, current_val);
    }
}
