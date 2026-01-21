use xprec::Df64;

fn main() {
    let n: u64 = 100_000_000; // 1億項
    let exact = std::f64::consts::PI.powi(2) / 6.0;

    let mut sum_f64: f64 = 0.0;
    let mut sum_df64: Df64 = Df64::ZERO;

    for i in 1..=n {
        let i_f64 = i as f64;
        let term = 1.0 / (i_f64 * i_f64);
        sum_f64 += term;
        sum_df64 += Df64::from(term);
    }

    // 級数の打ち切り誤差を補正（1/N で近似）
    let correction = 1.0 / (n as f64);
    sum_f64 += correction;
    sum_df64 += Df64::from(correction);

    println!("解析解:       {:.16}", exact);
    println!("f64  の結果:  {:.16}", sum_f64);
    println!("Df64 の結果:  {:.16}", sum_df64.hi());
    println!("f64  の誤差:  {:.2e}", (sum_f64 - exact).abs());
    println!("Df64 の誤差:  {:.2e}", (sum_df64.hi() - exact).abs());
}
