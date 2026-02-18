use rand::RngExt;

fn main() {
    println!("目標分布: 標準正規分布 N(0, 1)");
    println!("提案分布: 一様ランダムウォーク");
    println!("受理確率: A = min(1, p(x')/p(x))\n");

    let mut rng = rand::rngs::ThreadRng::default();

    // パラメータ設定
    let x_init = 10.0; // 分布の中心から遠い初期値（バーンインの効果を見るため）
    let delta = 1.0; // 提案のステップ幅
    let n_steps = 100_000;
    let burn_in = n_steps / 10; // 最初の10%をバーンイン期間とする

    // 目標分布（規格化定数は不要）
    let p_unnormalized = |x: f64| (-0.5 * x * x).exp();

    println!("パラメータ:");
    println!("  初期値 x₀ = {}", x_init);
    println!("  ステップ幅 δ = {}", delta);
    println!("  総ステップ数 = {}", n_steps);
    println!("  バーンイン期間 = {} ステップ\n", burn_in);

    // サンプリング実行
    let mut x = x_init;
    let mut samples = Vec::with_capacity(n_steps);
    let mut accepted = 0;

    for _ in 0..n_steps {
        // 1. 候補 x' を現在の位置 x の近傍 [-δ, δ] から提案
        let x_next = x + rng.random_range(-delta..delta);

        // 2. 受理確率 A = min(1, p(x') / p(x)) の計算
        // 比をとることで、規格化定数がキャンセルされる
        let ratio = p_unnormalized(x_next) / p_unnormalized(x);
        let acceptance_prob = ratio.min(1.0);

        // 3. 受理判定
        if rng.random::<f64>() < acceptance_prob {
            x = x_next;
            accepted += 1;
        }

        samples.push(x);
    }

    // 統計量の計算
    let acceptance_rate = (accepted as f64) / (n_steps as f64);

    // バーンイン後のサンプルで統計を計算
    let valid_samples = &samples[burn_in..];
    let n = valid_samples.len() as f64;
    let mean = valid_samples.iter().sum::<f64>() / n;
    let variance = valid_samples
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / n;
    let std_dev = variance.sqrt();

    // バーンイン前の統計（比較のため）
    let burnin_samples = &samples[0..burn_in];
    let mean_burnin = burnin_samples.iter().sum::<f64>() / (burn_in as f64);

    // 結果の表示
    println!("--- 結果 ---");
    println!("受理率: {:.2}%", acceptance_rate * 100.0);
    println!("  (理想的には 20-50% 程度が効率的)\n");

    println!("バーンイン期間の平均: {:.6}", mean_burnin);
    println!("  (初期値 {} から定常分布へ収束中)\n", x_init);

    println!("バーンイン後の統計 ({} サンプル):", valid_samples.len());
    println!("  平均:       {:.6}  (理論値: 0.0)", mean);
    println!("  標準偏差:   {:.6}  (理論値: 1.0)", std_dev);
    println!("  分散:       {:.6}  (理論値: 1.0)", variance);

    println!("\n理論値との誤差:");
    println!("  平均の誤差: {:.6}", mean.abs());
    println!("  標準偏差の誤差: {:.6}", (std_dev - 1.0).abs());
}
