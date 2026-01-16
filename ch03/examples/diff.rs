fn main() {
    // 微分対象の関数 f(x) = sin(x)
    let f = |x: f64| x.sin();

    // 微分を計算する点
    let x: f64 = 1.0;

    // 刻み幅 h
    let h: f64 = 0.01;

    // 解析解 (真値): cos(1.0)
    let exact = x.cos();

    // 1. 前進差分
    let diff_forward = (f(x + h) - f(x)) / h;

    // 2. 後退差分
    let diff_backward = (f(x) - f(x - h)) / h;

    // 3. 中心差分
    let diff_central = (f(x + h) - f(x - h)) / (2.0 * h);

    println!("解析解:    {:.10}", exact);
    println!(
        "前進差分:  {:.10} (誤差: {:.2e})",
        diff_forward,
        (diff_forward - exact).abs()
    );
    println!(
        "後退差分:  {:.10} (誤差: {:.2e})",
        diff_backward,
        (diff_backward - exact).abs()
    );
    println!(
        "中心差分:  {:.10} (誤差: {:.2e})",
        diff_central,
        (diff_central - exact).abs()
    );
}
