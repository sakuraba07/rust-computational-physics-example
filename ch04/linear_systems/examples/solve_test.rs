use ndarray::{arr1, arr2};
use ndarray_linalg::Solve;

fn main() {
    // 係数行列 A
    let a = arr2(&[[3.0, 1.0], [1.0, 2.0]]);

    // 右辺ベクトル b
    let b = arr1(&[9.0, 8.0]);

    // Ax = b を解く
    let x = a.solve(&b).expect("Failed to solve");

    println!("Solution x = {}", x);
    // 期待される解:
    // 3x + y = 9
    // x + 2y = 8
    // -> x=2, y=3
}
