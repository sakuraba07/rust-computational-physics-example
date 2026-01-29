use roots::find_root_brent;
use roots::SimpleConvergency;

fn main() {
    let f = |x: f64| x * x - 2.0;
    let mut convergency = SimpleConvergency { eps: 1e-15f64, max_iter: 30 };

    // 区間 [1.0, 2.0] で解を探す
    // find_root_brent(初期区間始点, 初期区間終点, 関数, 収束条件)
    let root = find_root_brent(1.0, 2.0, &f, &mut convergency);

    match root {
        Ok(val) => println!("解: {}", val),
        Err(e) => println!("エラー: {:?}", e),
    }
}
