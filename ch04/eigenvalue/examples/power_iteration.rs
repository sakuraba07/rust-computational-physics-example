use ndarray::{Array1, Array2, arr2};
use ndarray_linalg::Norm;

fn power_iteration(a: &Array2<f64>, max_iter: usize, tol: f64) -> (f64, Array1<f64>) {
    let n = a.nrows();
    // 初期ベクトル（ランダムまたは適当な値）
    let mut x = Array1::from_elem(n, 1.0);
    x = &x / x.norm_l2(); // 正規化

    let mut eigenvalue = 0.0;

    for _ in 0..max_iter {
        let x_next = a.dot(&x);
        let x_next_norm = x_next.norm_l2();

        // レイリー商による固有値の推定: λ ≈ xᵀAx / xᵀx
        // ここでは単純にノルムの比に近いが、厳密には内積をとる方が精度が良い
        let next_val = x.dot(&x_next);

        // 収束判定
        if (next_val - eigenvalue).abs() < tol {
            eigenvalue = next_val;
            x = &x_next / x_next_norm;
            break;
        }

        eigenvalue = next_val;
        x = &x_next / x_next_norm;
    }

    (eigenvalue, x)
}

fn main() {
    let a = arr2(&[[2.0, 1.0], [1.0, 3.0]]);

    let (eval, evec) = power_iteration(&a, 1000, 1e-6);

    println!("Dominant Eigenvalue: {:.5}", eval);
    println!("Eigenvector: {}", evec);

    // 理論値:
    // trace=5, det=5 -> λ² - 5λ + 5 = 0
    // λ = (5 ± √(25 - 20))/2 = (5 ± 2.236)/2 = 3.618, 1.382
    // 最大固有値は約 3.618
}
