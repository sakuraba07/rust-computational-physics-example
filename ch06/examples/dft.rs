use ndarray::{Array1, Array2, ArrayView1};
use num_complex::Complex64;
use std::f64::consts::PI;

/// 定義式に基づいたDFTの直接計算
fn dft(x: ArrayView1<Complex64>) -> Array1<Complex64> {
    let n = x.len();
    let mut x_k = Array1::zeros(n);

    for k in 0..n {
        let mut sum = Complex64::new(0.0, 0.0);
        for n_idx in 0..n {
            let angle = -2.0 * PI * (k as f64) * (n_idx as f64) / (n as f64);
            // オイラーの公式 exp(iθ) = cos θ + i sin θ を用いた計算
            let exponent = Complex64::from_polar(1.0, angle);
            sum += x[n_idx] * exponent;
        }
        x_k[k] = sum;
    }
    x_k
}

fn dft_matrix_method(x: &Array1<Complex64>) -> Array1<Complex64> {
    let n = x.len();
    // DFT行列の作成
    let mut w = Array2::<Complex64>::zeros((n, n));
    for k in 0..n {
        for n_idx in 0..n {
            let angle = -2.0 * PI * (k as f64) * (n_idx as f64) / (n as f64);
            w[[k, n_idx]] = Complex64::from_polar(1.0, angle);
        }
    }
    // 行列とベクトルの積
    w.dot(x)
}

fn main() {
    // 例：4点のデータ
    let data = Array1::from(vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(2.0, 0.0),
        Complex64::new(3.0, 0.0),
        Complex64::new(4.0, 0.0),
    ]);

    let result_1 = dft(data.view());
    let result_2 = dft_matrix_method(&data);

    println!("DFT Result (Direct Calculation):");
    for (k, val) in result_1.iter().enumerate() {
        println!("X[{}] = {:.3} + {:.3}i", k, val.re, val.im);
    }

    println!("\nDFT Result (Matrix Method):");
    for (k, val) in result_2.iter().enumerate() {
        println!("X[{}] = {:.3} + {:.3}i", k, val.re, val.im);
    }
}
