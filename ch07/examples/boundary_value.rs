use ndarray::{Array1, arr1};
use std::f64::consts::PI;

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

/// 初期値問題 (IVP) として t0 から t1 まで積分し、終端の状態を返す
fn solve_ivp(v0: f64) -> f64 {
    let system = |_t: f64, state: &Array1<f64>| arr1(&[state[1], -state[0]]);
    let mut state = arr1(&[0.0, v0]); // x(0)=0, v(0)=v0
    let mut t = 0.0;
    let t1 = PI / 2.0;
    let h = 0.01;

    while t < t1 {
        let step_h = if t + h > t1 { t1 - t } else { h };
        state = rk4_step(&state, t, step_h, system);
        t += step_h;
    }
    state[0] // 終端位置 x(t1) を返す
}

fn main() {
    let target_x1 = 1.0; // 目標: x(pi/2) = 1
    let tolerance = 1e-8;

    // 二分法による初期速度 v0 の探索
    let mut low = 0.0;
    let mut high = 2.0;
    let mut v0 = (low + high) / 2.0;

    println!("{:<5} {:<15} {:<15}", "Iter", "v0 (Guess)", "x(pi/2) Error");
    println!("{}", "-".repeat(40));

    for i in 0..50 {
        let x_final = solve_ivp(v0);
        let error = x_final - target_x1;

        println!("{:<5} {:<15.8} {:<15.2e}", i, v0, error);

        if error.abs() < tolerance {
            break;
        }

        if error < 0.0 {
            low = v0;
        } else {
            high = v0;
        }
        v0 = (low + high) / 2.0;
    }

    println!("\n結果: 求める初期速度 v(0) = {:.8}", v0);
}
