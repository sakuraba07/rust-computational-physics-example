use rand::RngExt;
use rayon::prelude::*;

fn main() {
    let m = 100_000_000;

    // 並列イテレータによる集計
    let hits: u64 = (0..m)
        .into_par_iter()
        .map_init(
            rand::rngs::ThreadRng::default, // 各スレッドの初期化時に一度だけ呼ばれる
            |rng, _| {
                // 各要素の処理で呼ばれる
                let x: f64 = rng.random();
                let y: f64 = rng.random();
                if x * x + y * y <= 1.0 { 1 } else { 0 }
            },
        )
        .sum();

    let pi_est = 4.0 * (hits as f64) / (m as f64);
    println!("Estimated pi = {:.8}", pi_est);
}
