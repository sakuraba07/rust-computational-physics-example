use ndarray::arr2;

fn main() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    let trace = a.diag().sum();
    println!("Trace: {}", trace); // 1.0 + 4.0 = 5.0
}
