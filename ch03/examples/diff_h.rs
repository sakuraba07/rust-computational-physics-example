use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("diff_error.csv")?;
    let _ = writeln!(file, "h,error");

    let f = |x: f64| x.sin();
    let x: f64 = 1.0;
    let exact = x.cos();

    println!("h,      Error (Central)");

    let mut h = 1.0;
    for _ in 0..16 {
        h /= 10.0;
        let approx = (f(x + h) - f(x - h)) / (2.0 * h);
        let error = (approx - exact).abs();
        println!("{:.1e}, {:.2e}", h, error);
        writeln!(file, "{:.1e},{:.2e}", h, error)?;
    }

    println!("diff_error.csv を作成しました");
    Ok(())
}
