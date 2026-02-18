use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Exp, Gamma, Normal};

fn main() {
    println!("=== 乱数生成のデモ ===\n");

    // 1. 固定シード（再現性のある乱数）
    demonstrate_fixed_seed();

    println!();

    // 2. 各種分布からのサンプリング
    demonstrate_distributions();

    println!();

    // 3. 統計的性質の確認
    demonstrate_statistics();
}

/// 固定シードによる再現可能な乱数生成
/// 物理シミュレーションのデバッグや結果の再現に重要
fn demonstrate_fixed_seed() {
    println!("--- 1. 固定シードによる再現可能な乱数 ---");

    // 32バイトのシード（種）を固定することで、常に同じ乱数列を得る
    let seed = [0u8; 32];
    // シードから生成器を初期化
    let mut rng = ChaCha8Rng::from_seed(seed);

    // [0, 1) の範囲の f64 型の一様乱数を生成
    let x: f64 = rng.random();
    // 1 から 100 までの整数の範囲から一様乱数を生成
    let n: i32 = rng.random_range(1..=100);

    println!("固定シードからの一様乱数: x = {:.6}, n = {}", x, n);
    println!("（同じシードを使えば、常に同じ値が得られます）");
}

/// 各種確率分布からのサンプリング
/// 物理現象のモデリングに応じて適切な分布を選択する
fn demonstrate_distributions() {
    println!("--- 2. 各種確率分布からのサンプリング ---");

    let mut rng = rand::rngs::ThreadRng::default();

    // 正規分布（ガウス分布）: N(平均 mu, 標準偏差 sigma)
    // 用途: 測定誤差、熱雑音、ブラウン運動の変位など
    let normal = Normal::new(0.0, 1.0).unwrap();
    let v_n = normal.sample(&mut rng);
    println!("正規分布 N(0, 1): {:.6}", v_n);
    println!("  用途例: 測定誤差、熱雑音、中心極限定理が成り立つ現象");

    // 指数分布: Exp(lambda)
    // 用途: 放射性崩壊の待ち時間、光子の自由行程、ポアソン過程のイベント間隔
    let exp = Exp::new(1.0).unwrap();
    let v_e = exp.sample(&mut rng);
    println!("\n指数分布 Exp(1.0): {:.6}", v_e);
    println!("  用途例: 放射性崩壊の待ち時間、粒子の平均自由行程");

    // ガンマ分布: Gamma(shape, scale)
    let gamma = Gamma::new(2.0, 2.0).unwrap();
    let v_g = gamma.sample(&mut rng);
    println!("\nガンマ分布 Gamma(2.0, 2.0): {:.6}", v_g);
    println!("  用途例: 複数イベントの待ち時間、ベイズ統計");
}

/// 統計的性質の確認
/// 大量のサンプルから分布の特性（平均、分散）を計算
fn demonstrate_statistics() {
    println!("--- 3. 統計的性質の確認（N=10000サンプル）---");

    let mut rng = rand::rngs::ThreadRng::default();
    let n_samples = 10000;

    // 正規分布 N(0, 1) のサンプリング
    let normal = Normal::new(0.0, 1.0).unwrap();
    let samples_normal: Vec<f64> = (0..n_samples).map(|_| normal.sample(&mut rng)).collect();

    let (mean_n, std_n) = compute_statistics(&samples_normal);
    println!("正規分布 N(0, 1):");
    println!("  理論値: 平均 = 0.0, 標準偏差 = 1.0");
    println!("  実測値: 平均 = {:.6}, 標準偏差 = {:.6}", mean_n, std_n);

    // 指数分布 Exp(1.0) のサンプリング
    let exp = Exp::new(1.0).unwrap();
    let samples_exp: Vec<f64> = (0..n_samples).map(|_| exp.sample(&mut rng)).collect();

    let (mean_e, std_e) = compute_statistics(&samples_exp);
    println!("\n指数分布 Exp(1.0):");
    println!("  理論値: 平均 = 1.0, 標準偏差 = 1.0");
    println!("  実測値: 平均 = {:.6}, 標準偏差 = {:.6}", mean_e, std_e);

    // 一様分布 [0, 1) のサンプリング
    let samples_uniform: Vec<f64> = (0..n_samples).map(|_| rng.random::<f64>()).collect();

    let (mean_u, std_u) = compute_statistics(&samples_uniform);
    println!("\n一様分布 U(0, 1):");
    println!(
        "  理論値: 平均 = 0.5, 標準偏差 = {:.6}",
        (1.0 / 12.0_f64).sqrt()
    );
    println!("  実測値: 平均 = {:.6}, 標準偏差 = {:.6}", mean_u, std_u);
}

/// サンプルから平均と標準偏差を計算
fn compute_statistics(samples: &[f64]) -> (f64, f64) {
    let n = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let variance = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    (mean, std_dev)
}
