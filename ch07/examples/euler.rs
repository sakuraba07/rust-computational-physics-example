/// オイラー法による1ステップの更新
fn euler_step<F>(x: f64, t: f64, h: f64, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    x + h * f(t, x)
}

fn main() {
    let f = |_t: f64, x: f64| -x;
    let x0 = 1.0;
    let t_max = 2.0;

    println!(
        "{:<5} {:<15} {:<15} {:<15}",
        "h", "Numerical", "Exact", "Error"
    );
    println!("{}", "-".repeat(55));

    for &h in &[0.4, 0.2, 0.1, 0.05] {
        let mut t = 0.0;
        let mut x = x0;

        while t < t_max {
            x = euler_step(x, t, h, f);
            t += h;
        }

        let exact = (-t_max).exp();
        println!(
            "{:<5.2} {:<15.8} {:<15.8} {:<15.2e}",
            h,
            x,
            exact,
            (x - exact).abs()
        );
    }
}
