use ndarray::Array2;

fn main() {
    let n = 50; // グリッドサイズ 50x50
    let max_iter = 10000;
    let tolerance = 1e-4; // 収束判定の閾値

    // 2次元グリッドの初期化 (0.0)
    let mut phi = Array2::<f64>::zeros((n, n));

    // 境界条件の設定
    // 上辺 (y=0) を 100.0 に固定
    for x in 0..n {
        phi[[0, x]] = 100.0;
    }
    // 左辺、右辺、下辺は 0.0 のまま

    for iter in 0..max_iter {
        let mut max_diff = 0.0;

        // グリッド内部の更新 (y, x)
        for y in 1..n - 1 {
            for x in 1..n - 1 {
                let old_val = phi[[y, x]];

                // ガウス＝ザイデル法: 最新の値をそのまま使って更新
                let new_val =
                    0.25 * (phi[[y, x + 1]] + phi[[y, x - 1]] + phi[[y + 1, x]] + phi[[y - 1, x]]);

                phi[[y, x]] = new_val;

                let diff = (new_val - old_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                }
            }
        }

        // 収束判定
        if max_diff < tolerance {
            println!("収束しました: 反復回数 {}", iter + 1);
            break;
        }

        if iter % 500 == 0 {
            println!("Iter {}: max_diff = {:.6}", iter, max_diff);
        }
    }

    // 結果の確認（中心付近の値）
    println!("phi[25, 25] = {:.2}", phi[[n / 2, n / 2]]);
}
