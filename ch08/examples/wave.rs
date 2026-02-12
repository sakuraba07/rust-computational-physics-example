use ndarray::Array1;

fn main() {
    let nx = 100;
    let nt = 300;
    let dx = 0.1;
    let dt = 0.05;
    let v = 1.0; // 波の速度

    // CFL条件のチェック
    let c = v * dt / dx;
    println!("CFL数 = {:.3}", c);
    if c > 1.0 {
        eprintln!("Warning: 不安定な条件 (CFL > 1) です！");
    }

    // 3つの時間ステップを保持
    let mut u_prev = Array1::<f64>::zeros(nx); // u^(n-1)
    let mut u_curr = Array1::<f64>::zeros(nx); // u^n
    let mut u_next = Array1::<f64>::zeros(nx); // u^(n+1)

    // 1. 初期条件の設定 (t=0)
    // ガウス波束を中心に配置
    let center = (nx / 2) as f64 * dx;
    let sigma = 1.0_f64;
    for i in 0..nx {
        let x = i as f64 * dx;
        u_curr[i] = (-(x - center).powi(2) / (2.0 * sigma.powi(2))).exp();
    }

    // 2. 最初のステップ (n=1) の計算
    // 初期速度 0 と仮定
    let c2 = c * c;
    for i in 1..nx - 1 {
        u_next[i] = u_curr[i] + 0.5 * c2 * (u_curr[i + 1] - 2.0 * u_curr[i] + u_curr[i - 1]);
    }
    // 境界条件 (固定端)
    u_next[0] = 0.0;
    u_next[nx - 1] = 0.0;

    // バッファの更新
    u_prev.assign(&u_curr);
    u_curr.assign(&u_next);

    // 3. 時間発展ループ (n=2, 3, ...)
    for n in 2..nt {
        for i in 1..nx - 1 {
            u_next[i] = 2.0 * u_curr[i] - u_prev[i]
                + c2 * (u_curr[i + 1] - 2.0 * u_curr[i] + u_curr[i - 1]);
        }

        // 境界条件
        u_next[0] = 0.0;
        u_next[nx - 1] = 0.0;

        if n % 50 == 0 {
            println!("Step {}: u[center] = {:.4}", n, u_next[nx / 2]);
        }

        // バッファの更新 (値をコピーせずに代入したい場合は工夫が必要だが、ここでは単純に)
        u_prev.assign(&u_curr);
        u_curr.assign(&u_next);
    }
}
