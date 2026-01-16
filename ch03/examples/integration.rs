use std::f64::consts::PI;

/// 台形則
fn trapezoidal_rule<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let sum: f64 = (1..n).map(|i| f(a + i as f64 * h)).sum();

    // 両端の点は重み 1/2
    h * (0.5 * f(a) + sum + 0.5 * f(b))
}

/// シンプソン則 (nは偶数でなければならない)
fn simpsons_rule<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    assert!(
        n.is_multiple_of(2),
        "Simpson's rule requires an even number of intervals."
    );

    let h = (b - a) / n as f64;

    let mut sum_odd = 0.0;
    let mut sum_even = 0.0;

    for i in 1..n {
        let x = a + i as f64 * h;
        if i % 2 == 0 {
            sum_even += f(x);
        } else {
            sum_odd += f(x);
        }
    }

    h / 3.0 * (f(a) + 4.0 * sum_odd + 2.0 * sum_even + f(b))
}

fn main() {
    let f = |x: f64| x.sin();
    let a = 0.0;
    let b = PI;
    let exact = 2.0;

    println!(
        "{:<10} {:<20} {:<20} {:<20} {:<20}",
        "N", "Trapezoidal", "Error (Trap)", "Simpson", "Error (Simp)"
    );
    println!("{}", "-".repeat(95));

    let n_values = [10, 20, 40, 80, 160];

    for &n in &n_values {
        let trap = trapezoidal_rule(f, a, b, n);
        let simp = simpsons_rule(f, a, b, n);

        println!(
            "{:<10} {:.15}    {:.5e}           {:.15}    {:.5e}",
            n,
            trap,
            (trap - exact).abs(),
            simp,
            (simp - exact).abs()
        );
    }
}
