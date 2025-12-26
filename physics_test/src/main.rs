fn main() {
    // 物理定数
    let g = 9.8; // 重力加速度 [m/s^2]
    let v0 = 0.0; // 初速度 [m/s]
    let h0 = 100.0; // 初期高度 [m]

    println!("自由落下シミュレーション");
    println!("初期高度: {} m", h0);
    println!("重力加速度: {} m/s^2", g);
    println!();

    // 時刻ごとの位置と速度を計算
    println!("時刻[s]  高度[m]  速度[m/s]");
    println!("================================");

    for i in 0..=10 {
        let t = i as f64 * 0.5; // 0.5秒刻み
        let v = v0 - g * t; // 速度 v = v0 - gt
        let h = h0 + v0 * t - 0.5 * g * t * t; // 位置 h = h0 + v0*t - (1/2)*g*t^2

        if h >= 0.0 {
            println!("{:6.1}  {:7.2}  {:9.2}", t, h, v);
        } else {
            // 地面に到達したら、正確な到達時刻と速度を計算して終了
            let t_impact = (v0 + (v0 * v0 + 2.0 * g * h0).sqrt()) / g;
            let v_impact = v0 - g * t_impact;
            println!("{:6.2}  {:7.2}  {:9.2} (地面到達)", t_impact, 0.0, v_impact);
            break;
        }
    }
}
