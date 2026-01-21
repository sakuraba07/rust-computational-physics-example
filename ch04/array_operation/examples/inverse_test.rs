use ndarray::arr2;
use ndarray_linalg::Inverse;

fn main() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    let a_inv = a.inv().expect("Singular matrix");

    println!("Inverse matrix:\n{}", a_inv);

    // 確認: A * A⁻¹ = I (単位行列)
    println!("Check:\n{}", a.dot(&a_inv));
}
