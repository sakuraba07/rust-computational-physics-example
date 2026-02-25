use ndarray::{Array1, arr1};
use std::fs::File;
use std::io::Write;

// 天文単位系 (AU, Year, Solar Mass) では G*M = 4 * pi^2
const GM: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;

struct Planet {
    pos: Array1<f64>,
    vel: Array1<f64>,
}

impl Planet {
    fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self {
            pos: arr1(&[x, y]),
            vel: arr1(&[vx, vy]),
        }
    }

    // 全エネルギー E = K + U
    fn total_energy(&self) -> f64 {
        let v_sq = self.vel.dot(&self.vel);
        let r = self.pos.dot(&self.pos).sqrt();
        0.5 * v_sq - GM / r
    }

    // 角運動量 L = r x v (2次元ではスカラー)
    fn angular_momentum(&self) -> f64 {
        self.pos[0] * self.vel[1] - self.pos[1] * self.vel[0]
    }
}

fn compute_acceleration(pos: &Array1<f64>) -> Array1<f64> {
    let r_sq = pos.dot(pos);
    let r_inv_cb = 1.0 / (r_sq * r_sq.sqrt());
    -GM * r_inv_cb * pos
}

fn velocity_verlet_step(planet: &mut Planet, dt: f64) {
    let a_curr = compute_acceleration(&planet.pos);

    // 1. 位置更新
    planet.pos += &(&planet.vel * dt + 0.5 * &a_curr * dt * dt);

    // 2. 新しい加速度
    let a_next = compute_acceleration(&planet.pos);

    // 3. 速度更新
    planet.vel += &(0.5 * (&a_curr + &a_next) * dt);
}

fn main() {
    // 地球の初期条件 (r=1.0 AU, v=2*pi AU/yr)
    let mut earth = Planet::new(1.0, 0.0, 0.0, 2.0 * std::f64::consts::PI);
    let dt = 0.001; // 約8時間の刻み幅

    // CSVファイルを開く
    let csv_filename = "kepler.csv";
    let mut csv_file = File::create(csv_filename).expect("CSVファイルの作成に失敗しました");

    // CSVヘッダーを書き込む
    writeln!(csv_file, "time,x,y,vx,vy,energy,angular_momentum")
        .expect("CSVヘッダーの書き込みに失敗しました");

    println!("=== Kepler問題のシミュレーション (Velocity Verlet法) ===");
    println!(
        "Initial Position: ({:.4}, {:.4}) AU",
        earth.pos[0], earth.pos[1]
    );
    println!(
        "Initial Velocity: ({:.4}, {:.4}) AU/yr",
        earth.vel[0], earth.vel[1]
    );
    println!("Initial Energy: {:.6}", earth.total_energy());
    println!("Initial Angular Momentum: {:.6}", earth.angular_momentum());
    println!("\nTime, X, Y, Energy, L");
    println!("----------------------------------------");

    for i in 0..2000 {
        let t = i as f64 * dt;

        // CSVに全データを書き込む
        writeln!(
            csv_file,
            "{},{},{},{},{},{},{}",
            t,
            earth.pos[0],
            earth.pos[1],
            earth.vel[0],
            earth.vel[1],
            earth.total_energy(),
            earth.angular_momentum()
        )
        .expect("CSVデータの書き込みに失敗しました");

        if i % 10 == 0 {
            println!(
                "{:.3}, {:.4}, {:.4}, {:.6}, {:.6}",
                t,
                earth.pos[0],
                earth.pos[1],
                earth.total_energy(),
                earth.angular_momentum()
            );
        }
        velocity_verlet_step(&mut earth, dt);
    }

    println!("\n=== シミュレーション完了 ===");
    println!("結果を '{}' に保存しました", csv_filename);
}
