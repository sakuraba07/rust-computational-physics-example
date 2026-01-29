fn main() {
    // 解きたい関数: f(x) = x^2 - 2
    let f = |x: f64| x * x - 2.0;

    // 初期区間 [a, b]
    // f(1.0) = -1.0, f(2.0) = 2.0 なので、この間に解がある
    let mut a = 1.0;
    let mut b = 2.0;

    let tolerance = 1e-8; // 許容誤差
    let max_iter = 100;   // 最大反復回数（無限ループ防止）

    for i in 0..max_iter {
        let c = (a + b) / 2.0;
        let fc = f(c);

        if fc.abs() < tolerance || (b - a).abs() < tolerance {
            println!("解が見つかりました: x = {:.10} (反復回数: {})", c, i + 1);
            return;
        }

        // f(a) * f(c) < 0 なら左側に解がある
        if f(a) * fc < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    println!("収束しませんでした。");
}
