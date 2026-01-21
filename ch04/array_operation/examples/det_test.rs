use ndarray::arr2;
use ndarray_linalg::Determinant;

fn main() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    // det(A) = 1*4 - 2*3 = -2
    let det = a.det().unwrap();
    println!("Determinant: {}", det);
}
