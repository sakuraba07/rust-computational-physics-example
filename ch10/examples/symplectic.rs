use ndarray::{Array1, arr1};
use std::fs::File;
use std::io::Write;

struct Particle {
    pos: Array1<f64>,
    vel: Array1<f64>,
}

impl Particle {
    fn new(x: f64, v: f64) -> Self {
        Self {
            pos: arr1(&[x]),
            vel: arr1(&[v]),
        }
    }

    // 全エネルギー E = 1/2 v^2 + 1/2 x^2
    fn energy(&self) -> f64 {
        0.5 * (self.vel[0].powi(2) + self.pos[0].powi(2))
    }
}

fn get_acceleration(pos: &Array1<f64>) -> Array1<f64> {
    -pos // 復元力 F = -x
}

fn velocity_verlet_step(p: &mut Particle, dt: f64) {
    let a_curr = get_acceleration(&p.pos);

    // 1. 位置の更新: x(t+dt) = x(t) + v(t)dt + 0.5*a(t)dt^2
    p.pos += &(&p.vel * dt + 0.5 * &a_curr * dt * dt);

    // 2. 新しい位置での加速度 a(t+dt)
    let a_next = get_acceleration(&p.pos);

    // 3. 速度の更新: v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))dt
    p.vel += &(0.5 * (&a_curr + &a_next) * dt);
}

fn main() {
    let mut p = Particle::new(1.0, 0.0);
    let dt = 0.1;

    // CSVファイルを開く
    let csv_filename = "symplectic.csv";
    let mut csv_file = File::create(csv_filename).expect("CSVファイルの作成に失敗しました");

    // CSVヘッダーを書き込む
    writeln!(csv_file, "time,position,velocity,energy")
        .expect("CSVヘッダーの書き込みに失敗しました");

    println!("=== Velocity Verlet法によるシンプレクティック積分 ===");
    println!(
        "Initial Position: {:.4}, Initial Velocity: {:.4}",
        p.pos[0], p.vel[0]
    );
    println!("Initial Energy: {:.6}", p.energy());
    println!("\nTime, Position, Velocity, Energy");
    println!("----------------------------------------");

    for i in 0..101 {
        let t = i as f64 * dt;

        // CSVに全データを書き込む
        writeln!(csv_file, "{},{},{},{}", t, p.pos[0], p.vel[0], p.energy())
            .expect("CSVデータの書き込みに失敗しました");

        if i % 10 == 0 {
            println!(
                "{:.1}, {:.4}, {:.4}, {:.6}",
                t,
                p.pos[0],
                p.vel[0],
                p.energy()
            );
        }
        velocity_verlet_step(&mut p, dt);
    }

    println!("\n=== シミュレーション完了 ===");
    println!("結果を '{}' に保存しました", csv_filename);
}
