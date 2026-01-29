fn main() {
    let f = |x: f64| x * x - 2.0;
    let df = |x: f64| 2.0 * x; // f(x) の導関数

    let mut x = 1.0; // 初期値
    let tolerance = 1e-8;
    let max_iter = 100;

    for i in 0..max_iter {
        let fx = f(x);

        if fx.abs() < tolerance {
             println!("解が見つかりました: x = {:.10} (反復回数: {})", x, i);
             return;
        }

        let dfx = df(x);
        // 接線の傾きが0に近いと発散の危険がある
        if dfx.abs() < 1e-10 {
            println!("微分値が0に近づきました。");
            break;
        }

        x -= fx / dfx;
    }
}
