use ndarray::arr2;
use ndarray_linalg::Cholesky;
use ndarray_linalg::UPLO;

fn main() {
    // 対称正定値行列
    let a = arr2(&[[4.0, 1.0], [1.0, 4.0]]);

    // コレスキー分解 (Lower triangular)
    let l = a.cholesky(UPLO::Lower).expect("Cholesky failed");

    println!("L =\n{}", l);
    println!("L * L^T =\n{}", l.dot(&l.t()));
    // L * L^T = A となるはず

    // 分解結果を使って方程式を解くことも可能
    // (APIの詳細はバージョンによりますが、通常 solve メソッドなどが提供されます)
}
