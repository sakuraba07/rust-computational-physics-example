use ndarray::{arr1, arr2};
use ndarray_linalg::Solve;

fn main() {
    // 初期値 (x, y) = (1.0, 2.0)
    // 解に近い適切な初期値を選ぶ必要があります
    let mut x = arr1(&[1.0, 2.0]);

    let tolerance = 1e-8;
    let max_iter = 100;

    for i in 0..max_iter {
        // 現在の x, y
        let curr_x: f64 = x[0];
        let curr_y: f64 = x[1];

        // 残差ベクトル F(x)
        let f1 = curr_x.powi(2) + curr_y.powi(2) - 1.0;
        let f2 = curr_y - curr_x.powi(2);
        let f_vec = arr1(&[f1, f2]);

        // 収束判定 (ノルムが十分小さいか)
        if f_vec.iter().map(|v| v.powi(2)).sum::<f64>().sqrt() < tolerance {
            println!("解が見つかりました: x={:.6}, y={:.6} (反復: {})", curr_x, curr_y, i);
            return;
        }

        // ヤコビ行列 J(x)
        let j = arr2(&[
            [2.0 * curr_x, 2.0 * curr_y],
            [-2.0 * curr_x, 1.0       ]
        ]);

        // 連立一次方程式 J * delta_x = -F を解く
        // solve() は ndarray-linalg の機能
        let delta = j.solve(&(-f_vec)).expect("Singular Jacobian");

        // 更新
        x = x + delta;
    }

    println!("収束しませんでした");
}
