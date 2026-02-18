use rand::RngExt;

fn main() {
    let m = 1_000_000;
    let mut hits = 0;
    let mut rng = rand::rngs::ThreadRng::default();

    for _ in 0..m {
        // [0, 1) の範囲で一様にサンプリング
        let x: f64 = rng.random();
        let y: f64 = rng.random();

        // 単位円の内部 (x^2 + y^2 <= 1) にあるか判定
        if x * x + y * y <= 1.0 {
            hits += 1;
        }
    }

    // 正方形の面積 (1.0) に対する円の面積 (pi/4) の比を利用
    // (hits / m) approx (pi / 4)  =>  pi approx 4 * (hits / m)
    let pi_est = 4.0 * (hits as f64) / (m as f64);
    println!("Estimated pi = {:.6}", pi_est);
}
