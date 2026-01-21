use ndarray::{arr1, arr2};
use ndarray_linalg::{Factorize, Solve}; // LU分解のために必要

fn main() {
    let a = arr2(&[[3.0, 1.0], [1.0, 2.0]]);

    // LU分解を実行
    let f = a.factorize().expect("Factorization failed");

    // 1つ目の b に対して解く
    let b1 = arr1(&[9.0, 8.0]);
    let x1 = f.solve(&b1).expect("Failed to solve b1");
    println!("x1 = {}", x1);

    // 2つ目の b に対して解く（LU分解の結果を再利用するため高速）
    // 3x + y = 4, x + 2y = 3 -> x = 1, y = 1
    let b2 = arr1(&[4.0, 3.0]);
    let x2 = f.solve(&b2).expect("Failed to solve b2");
    println!("x2 = {}", x2);
}
