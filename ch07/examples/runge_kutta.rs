use ndarray::{Array1, arr1};

/// 4次のルンゲ＝クッタ法による1ステップの更新
fn rk4_step<F>(state: &Array1<f64>, t: f64, h: f64, f: F) -> Array1<f64>
where
    F: Fn(f64, &Array1<f64>) -> Array1<f64>,
{
    let k1 = f(t, state);
    let k2 = f(t + h * 0.5, &(state + &k1 * (h * 0.5)));
    let k3 = f(t + h * 0.5, &(state + &k2 * (h * 0.5)));
    let k4 = f(t + h, &(state + &k3 * h));

    state + (&k1 + &k2 * 2.0 + &k3 * 2.0 + &k4) * (h / 6.0)
}

fn main() {
    // dx/dt = v, dv/dt = -x
    let system = |_t: f64, state: &Array1<f64>| -> Array1<f64> {
        let x = state[0];
        let v = state[1];
        arr1(&[v, -x])
    };

    let x0 = arr1(&[1.0, 0.0]); // 初期条件: x=1, v=0
    let t_max = 2.0 * std::f64::consts::PI; // 1周期

    println!("{:<5} {:<15} {:<15}", "h", "Final x", "Error");
    println!("{}", "-".repeat(40));

    for &h in &[0.5, 0.25, 0.125, 0.0625] {
        let mut t = 0.0;
        let mut state = x0.clone();

        while t < t_max {
            // ステップ幅が余る場合の調整
            let step_h = if t + h > t_max { t_max - t } else { h };
            state = rk4_step(&state, t, step_h, system);
            t += step_h;
        }

        let exact = 1.0; // cos(2pi) = 1
        println!(
            "{:<5.3} {:<15.10} {:<15.2e}",
            h,
            state[0],
            (state[0] - exact).abs()
        );
    }
}
