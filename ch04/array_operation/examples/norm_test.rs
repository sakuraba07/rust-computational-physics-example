use ndarray::{arr1, arr2};
use ndarray_linalg::{Norm, OperationNorm};

fn main() {
    let x = arr1(&[3.0, 4.0]);

    // L2ノルム: √(3² + 4²) = 5.0
    println!("L2 norm: {}", x.norm_l2());

    // L1ノルム: |3| + |4| = 7.0
    println!("L1 norm: {}", x.norm_l1());

    // 最大値ノルム: max(|3|, |4|) = 4.0
    println!("Max norm: {}", x.norm_max());

    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    let norm_one = a.opnorm_one().unwrap();
    let norm_inf = a.opnorm_inf().unwrap();
    let norm_fro = a.opnorm_fro().unwrap();

    // 行列のL2演算子ノルム
    println!("Operator L2 1-norm: {}", norm_one);
    println!("Operator L2 infinity norm: {}", norm_inf);
    println!("Operator L2 Frobenius norm: {}", norm_fro);
}
