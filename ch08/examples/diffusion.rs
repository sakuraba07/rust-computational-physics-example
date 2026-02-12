use ndarray::Array1;

fn main() {
    let nx = 50; // 空間分割数
    let nt = 500; // 時間ステップ数
    let dx = 1.0;
    let dt = 0.2;
    let d_coeff = 1.0; // 拡散係数

    let r = d_coeff * dt / (dx * dx);
    println!("拡散数 r = {:.3}", r);

    if r > 0.5 {
        eprintln!("Warning: 安定性条件 (r <= 0.5) を満たしていません！");
    }

    // 初期状態: 中央に熱源がある（デルタ関数的な初期分布）
    let mut u = Array1::<f64>::zeros(nx);
    u[nx / 2] = 100.0;

    // 時間発展ループ
    for n in 0..nt {
        let mut u_next = Array1::<f64>::zeros(nx);

        // 境界を除く内部点を更新 (スライスを用いた並列化も可能)
        for i in 1..nx - 1 {
            u_next[i] = u[i] + r * (u[i + 1] - 2.0 * u[i] + u[i - 1]);
        }

        // 境界条件 (固定境界: ディリクレ条件)
        u_next[0] = 0.0;
        u_next[nx - 1] = 0.0;

        u = u_next;

        // 100ステップごとに中央の値を表示
        if n % 100 == 0 {
            println!("Step {}: u[center] = {:.4}", n, u[nx / 2]);
        }
    }
}
