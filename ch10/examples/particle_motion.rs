use ndarray::{Array1, arr1};
use std::fs::File;
use std::io::Write;

struct FallingBody {
    m: f64, // 質量
    k: f64, // 空気抵抗係数
    g: f64, // 重力加速度
}

impl FallingBody {
    fn new(m: f64, k: f64) -> Self {
        Self { m, k, g: 9.8 }
    }

    /// 第7章のソルバーが期待する形式: f(t, y) -> dy/dt
    /// 状態ベクトル y = [x, y, vx, vy]
    fn dynamics(&self, _t: f64, y: &Array1<f64>) -> Array1<f64> {
        // 現在の速度を取り出す
        let vx = y[2];
        let vy = y[3];

        // 加速度の計算: a = F/m = (重力 + 空気抵抗) / m
        // ax = - (k/m) * vx
        let ax = -(self.k / self.m) * vx;
        // ay = -g - (k/m) * vy
        let ay = -self.g - (self.k / self.m) * vy;

        // dy/dt = [dx/dt, dy/dt, dvx/dt, dvy/dt] = [vx, vy, ax, ay] を返す
        arr1(&[vx, vy, ax, ay])
    }
}

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
    let model = FallingBody::new(1.0, 0.1);

    // 初期状態: 原点から初速 (10.0, 15.0) で投げ出された状態
    let mut y = arr1(&[0.0, 0.0, 10.0, 15.0]);
    let mut t = 0.0;

    // シミュレーションパラメータ
    let dt = 0.01; // 時間刻み幅
    let t_max = 3.0; // 最大シミュレーション時間

    // CSVファイルを開く
    let csv_filename = "particle_motion.csv";
    let mut csv_file = File::create(csv_filename).expect("CSVファイルの作成に失敗しました");

    // CSVヘッダーを書き込む
    writeln!(csv_file, "time,x,y,vx,vy").expect("CSVヘッダーの書き込みに失敗しました");

    println!("=== Runge-Kutta法による粒子運動シミュレーション ===");
    println!("質量: {} kg, 空気抵抗係数: {}", model.m, model.k);
    println!(
        "初期状態: 位置=({:.2}, {:.2}), 速度=({:.2}, {:.2})",
        y[0], y[1], y[2], y[3]
    );
    println!("\n時刻\tx\ty\tvx\tvy");
    println!("----------------------------------------");

    // 初期状態を出力
    println!(
        "{:.2}\t{:.3}\t{:.3}\t{:.3}\t{:.3}",
        t, y[0], y[1], y[2], y[3]
    );

    // 初期状態をCSVに書き込む
    writeln!(csv_file, "{},{},{},{},{}", t, y[0], y[1], y[2], y[3])
        .expect("CSVデータの書き込みに失敗しました");

    // Runge-Kutta法で時間発展
    let mut step = 0;
    while t < t_max && y[1] >= 0.0 {
        // y座標が負にならない間シミュレーション
        // RK4法で1ステップ進める
        y = rk4_step(&y, t, dt, |t, state| model.dynamics(t, state));
        t += dt;
        step += 1;

        // CSVに全データを書き込む
        writeln!(csv_file, "{},{},{},{},{}", t, y[0], y[1], y[2], y[3])
            .expect("CSVデータの書き込みに失敗しました");

        // 10ステップごとに画面出力
        if step % 10 == 0 {
            println!(
                "{:.2}\t{:.3}\t{:.3}\t{:.3}\t{:.3}",
                t, y[0], y[1], y[2], y[3]
            );
        }

        // 地面に到達したら終了
        if y[1] < 0.0 {
            println!("\n地面に到達しました！");
            println!("最終時刻: {:.3} s", t);
            println!("最終位置: x={:.3} m, y={:.3} m", y[0], y[1]);
            println!("最終速度: vx={:.3} m/s, vy={:.3} m/s", y[2], y[3]);
            break;
        }
    }

    println!("\n=== シミュレーション完了 ===");
    println!("結果を '{}' に保存しました", csv_filename);
}
