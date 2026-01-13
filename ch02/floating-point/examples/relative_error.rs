use approx::assert_relative_eq;

fn main() {
    let a: f64 = 0.1 + 0.2;
    let b: f64 = 0.3;

    // a と b が相対誤差の範囲で等しいことを表明する
    assert_relative_eq!(a, b, max_relative = 1e-10);

    println!("approxクレートを使って a と b がほぼ等しいことを確認しました。");
}
