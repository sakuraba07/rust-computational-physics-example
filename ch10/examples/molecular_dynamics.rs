use ndarray::{Array1, Array2, Axis};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::fs::File;
use std::io::Write;

struct MDSystem {
    n: usize,
    l: f64,
    pos: Array2<f64>,
    vel: Array2<f64>,
    acc: Array2<f64>,
}

impl MDSystem {
    fn new(n: usize, l: f64, target_temp: f64) -> Self {
        let mut pos = Array2::zeros((n, 2));
        let n_side = (n as f64).sqrt() as usize;
        let spacing = l / n_side as f64;

        for i in 0..n {
            pos[[i, 0]] = (i % n_side) as f64 * spacing + spacing * 0.5;
            pos[[i, 1]] = (i / n_side) as f64 * spacing + spacing * 0.5;
        }

        // 1. ランダムな初速を与える（-0.5 ～ 0.5 の一様分布）
        let mut vel = Array2::<f64>::random((n, 2), Uniform::new(-0.5, 0.5).unwrap());

        // 2. 重心速度をゼロにする（系全体のドリフトを防ぐ）
        let mean_vel = vel.mean_axis(Axis(0)).unwrap();
        vel -= &mean_vel;

        // 3. 温度（運動エネルギー）の調整
        // 2次元の場合、自由度あたりのエネルギーから温度をスケーリング
        let current_temp = 0.5 * vel.mapv(|v: f64| v.powi(2)).sum() / n as f64;
        let scale = (target_temp / current_temp).sqrt();
        vel *= scale;

        Self {
            n,
            l,
            pos,
            vel,
            acc: Array2::zeros((n, 2)),
        }
    }

    fn get_dr(&self, i: usize, j: usize) -> Array1<f64> {
        let mut dr = &self.pos.row(i) - &self.pos.row(j);
        for k in 0..2 {
            if dr[k] > self.l * 0.5 {
                dr[k] -= self.l;
            } else if dr[k] < -self.l * 0.5 {
                dr[k] += self.l;
            }
        }
        dr
    }

    fn compute_forces(&mut self) -> f64 {
        self.acc.fill(0.0);
        let mut pot = 0.0;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                let dr = self.get_dr(i, j);
                let r2 = dr.dot(&dr);
                if r2 < 9.0 {
                    // カットオフ 3.0
                    let r2_inv = 1.0 / r2;
                    let r6_inv = r2_inv * r2_inv * r2_inv;
                    pot += 4.0 * (r6_inv * r6_inv - r6_inv);
                    let f_scalar = 24.0 * r2_inv * (2.0 * r6_inv * r6_inv - r6_inv);
                    for k in 0..2 {
                        self.acc[[i, k]] += f_scalar * dr[k];
                        self.acc[[j, k]] -= f_scalar * dr[k];
                    }
                }
            }
        }
        pot
    }

    fn step(&mut self, dt: f64) -> f64 {
        self.pos += &(&self.vel * dt + 0.5 * &self.acc * dt * dt);
        self.pos.mapv_inplace(|x| x.rem_euclid(self.l));
        let old_acc = self.acc.clone();
        let pot = self.compute_forces();
        self.vel += &(0.5 * (&old_acc + &self.acc) * dt);
        pot
    }
}

fn main() {
    // 16粒子、サイズ10.0の箱、温度0.5で初期化
    let mut system = MDSystem::new(16, 10.0, 0.5);
    let dt = 0.01;
    system.compute_forces();

    // CSVファイルを開く
    let csv_filename = "molecular_dynamics.csv";
    let mut csv_file = File::create(csv_filename).expect("CSVファイルの作成に失敗しました");

    // CSVヘッダーを書き込む
    writeln!(
        csv_file,
        "step,time,potential_energy,kinetic_energy,total_energy"
    )
    .expect("CSVヘッダーの書き込みに失敗しました");

    println!("=== 分子動力学シミュレーション ===");
    println!("粒子数: {}, 箱のサイズ: {:.1}", system.n, system.l);
    println!("時間刻み: {:.3}", dt);
    println!("\nStep, Potential, Kinetic, Total");
    println!("----------------------------------------");

    for i in 0..101 {
        let pot = system.step(dt);
        let kin = 0.5 * system.vel.mapv(|v: f64| v.powi(2)).sum();
        let total = pot + kin;
        let time = i as f64 * dt;

        // CSVに全データを書き込む
        writeln!(csv_file, "{},{},{},{},{}", i, time, pot, kin, total)
            .expect("CSVデータの書き込みに失敗しました");

        if i % 10 == 0 {
            println!("{:>4}, {:>10.4}, {:>10.4}, {:>10.4}", i, pot, kin, total);
        }
    }

    println!("\n=== シミュレーション完了 ===");
    println!("結果を '{}' に保存しました", csv_filename);
}
