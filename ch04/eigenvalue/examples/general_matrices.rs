use ndarray::arr2;
use ndarray_linalg::Eig;

fn main() {
    let a = arr2(&[[0.0, -1.0], [1.0, 0.0]]);

    // 固有値と固有ベクトルを計算
    let (evals, evecs) = a.eig().expect("Eig decomposition failed");

    println!("Eigenvalues: {}", evals);
    println!("Eigenvectors:\n{}", evecs);

    // 回転行列 [[0, -1], [1, 0]] の固有値は +/- i
    // 出力は Complex64 型になります
}
