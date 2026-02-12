use ndarray::{Array1, s};
use std::f64::consts::PI;

fn main() {
    let nx = 100;
    let x_min = 0.0;
    let x_max = 2.0 * PI;
    let dx = (x_max - x_min) / (nx - 1) as f64;

    // 1. 格子点の生成 (linspace)
    let x = Array1::linspace(x_min, x_max, nx);

    // 2. 関数の値を計算: u = sin(x)
    let u = x.mapv(|v| v.sin());

    // 3. 中心差分による数値微分を一括計算
    // du/dx[i] = (u[i+1] - u[i-1]) / (2*dx)
    // s![2..] は 2番目以降、s![..nx-2] は 最後から2つ目までを指す
    let du_dx_num = (&u.slice(s![2..]) - &u.slice(s![..nx - 2])) / (2.0 * dx);

    // 4. 解析解 (cos(x)) との比較
    let x_inner = x.slice(s![1..nx - 1]);
    let du_dx_exact = x_inner.mapv(|v| v.cos());

    // 中央付近（x = PI）の結果を表示
    let mid = nx / 2;

    println!("x = {:.4}", x_inner[mid]);
    println!("Numerical: {:.6}", du_dx_num[mid]);
    println!("Exact:     {:.6}", du_dx_exact[mid]);
    println!(
        "Error:     {:.2e}",
        (du_dx_num[mid] - du_dx_exact[mid]).abs()
    );
}
