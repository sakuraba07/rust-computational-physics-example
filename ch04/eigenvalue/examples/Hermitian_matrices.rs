use ndarray::arr2;
use ndarray_linalg::Eigh;
use ndarray_linalg::UPLO;

fn main() {
    // 対称行列（パウリ行列 sigma_x など）
    let a = arr2(&[[0.0, 1.0], [1.0, 0.0]]);

    // 固有値と固有ベクトルを計算
    // UPLO::Lower は下三角部分のみを参照することを意味します（対称なので）
    let (evals, evecs) = a.eigh(UPLO::Lower).expect("Eigh failed");

    println!("Eigenvalues: {}", evals); // [-1, 1]
    println!("Eigenvectors:\n{}", evecs);
}
