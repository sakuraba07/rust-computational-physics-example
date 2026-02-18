use rand::RngExt;
use rand_distr::{Distribution, Exp};

fn main() {
    // 目標: 以下の積分を数値的に計算する
    //   I = ∫₀^∞ f(x) dx = ∫₀^∞ e^(-x) / (1 + x²) dx
    //
    // 厳密解 (近似値): I ≈ 0.62144962
    //
    // この積分は解析的に計算困難だが、モンテカルロ法で近似できる。
    // ただし、x → ∞ での収束が遅いため、通常の一様サンプリングは非効率。
    // 重点サンプリングを用いることで、分散を大幅に削減できる。

    let n_samples = 1_000_000;
    let n_trials = 10; // 安定性を確認するための試行回数
    let exact_value = 0.62144962;

    println!("積分: ∫₀^∞ e^(-x) / (1 + x²) dx");
    println!("厳密解 (近似): {:.8}\n", exact_value);
    println!("サンプル数: {}\n", n_samples);

    // 複数回の試行結果を格納
    let mut uniform_results = Vec::new();
    let mut importance_results = Vec::new();

    for trial in 0..n_trials {
        let mut rng = rand::rngs::ThreadRng::default();

        // 被積分関数
        let f = |x: f64| (-x).exp() / (1.0 + x * x);

        // ----------------------------------------
        // 方法1: 一様サンプリング
        // ----------------------------------------
        // 積分区間 [0, ∞) を [0, L] で打ち切る
        // I ≈ L × E[f(X)]  where X ~ Uniform(0, L)
        let limit = 10.0;
        let mut sum_uniform = 0.0;
        let mut sum_sq_uniform = 0.0;

        for _ in 0..n_samples {
            let x = rng.random_range(0.0..limit);
            let fx = f(x);
            sum_uniform += fx;
            sum_sq_uniform += fx * fx;
        }

        let mean_uniform = sum_uniform / (n_samples as f64);
        let mean_sq_uniform = sum_sq_uniform / (n_samples as f64);
        let variance_uniform = mean_sq_uniform - mean_uniform * mean_uniform;
        let result_uniform = limit * mean_uniform;
        // 推定値の分散は limit^2 * variance となる
        let estimator_variance_uniform = limit * limit * variance_uniform;
        let std_error_uniform = (estimator_variance_uniform / (n_samples as f64)).sqrt();

        uniform_results.push((
            result_uniform,
            std_error_uniform,
            estimator_variance_uniform,
        ));

        // ----------------------------------------
        // 方法2: 重点サンプリング
        // ----------------------------------------
        // 提案分布: p(x) = e^(-x)  (指数分布、λ=1)
        // これは被積分関数 f(x) = e^(-x) / (1 + x²) の主要部分と一致
        //
        // モンテカルロ推定:
        //   I = ∫ f(x) dx = ∫ [f(x)/p(x)] p(x) dx = E_p[f(X)/p(X)]
        //
        // ここで X ~ p(x) = e^(-x) とサンプリングすると、
        //   f(x)/p(x) = [e^(-x) / (1 + x²)] / e^(-x) = 1 / (1 + x²)
        //
        // この比の形が単純で、分散が小さくなる。
        let exp_dist = Exp::new(1.0).unwrap();
        let mut sum_importance = 0.0;
        let mut sum_sq_importance = 0.0;

        for _ in 0..n_samples {
            let x = exp_dist.sample(&mut rng);
            // 重み w(x) = f(x) / p(x)
            let weight = 1.0 / (1.0 + x * x);
            sum_importance += weight;
            sum_sq_importance += weight * weight;
        }

        let mean_importance = sum_importance / (n_samples as f64);
        let mean_sq_importance = sum_sq_importance / (n_samples as f64);
        let variance_importance = mean_sq_importance - mean_importance * mean_importance;
        let result_importance = mean_importance;
        let std_error_importance = (variance_importance / (n_samples as f64)).sqrt();

        importance_results.push((result_importance, std_error_importance, variance_importance));

        if trial == 0 {
            println!("--- 試行 {} の詳細 ---", trial + 1);
            println!("\n[一様サンプリング (区間 [0, {}])]]", limit);
            println!("  推定値:     {:.8}", result_uniform);
            println!("  標準誤差:   {:.8}", std_error_uniform);
            println!("  推定値の分散: {:.8}", estimator_variance_uniform);
            println!("  誤差:       {:.8}", (result_uniform - exact_value).abs());

            println!("\n[重点サンプリング (p(x) = e^(-x))]");
            println!("  推定値:     {:.8}", result_importance);
            println!("  標準誤差:   {:.8}", std_error_importance);
            println!("  推定値の分散: {:.8}", variance_importance);
            println!(
                "  誤差:       {:.8}",
                (result_importance - exact_value).abs()
            );

            println!(
                "\n  分散削減率: {:.2}倍 (一様 {:.4} → 重点 {:.4})",
                estimator_variance_uniform / variance_importance,
                estimator_variance_uniform,
                variance_importance
            );
        }
    }

    // ----------------------------------------
    // 複数試行の統計
    // ----------------------------------------
    println!("\n\n=== {} 回の試行結果 ===", n_trials);

    let avg_uniform: f64 =
        uniform_results.iter().map(|(r, _, _)| r).sum::<f64>() / (n_trials as f64);
    let avg_importance: f64 =
        importance_results.iter().map(|(r, _, _)| r).sum::<f64>() / (n_trials as f64);

    let avg_var_uniform: f64 =
        uniform_results.iter().map(|(_, _, v)| v).sum::<f64>() / (n_trials as f64);
    let avg_var_importance: f64 =
        importance_results.iter().map(|(_, _, v)| v).sum::<f64>() / (n_trials as f64);

    println!("\n一様サンプリング:");
    println!("  平均推定値:     {:.8}", avg_uniform);
    println!("  平均推定値分散: {:.8}", avg_var_uniform);
    println!("  平均誤差:       {:.8}", (avg_uniform - exact_value).abs());

    println!("\n重点サンプリング:");
    println!("  平均推定値:     {:.8}", avg_importance);
    println!("  平均推定値分散: {:.8}", avg_var_importance);
    println!(
        "  平均誤差:       {:.8}",
        (avg_importance - exact_value).abs()
    );

    println!("\n改善効果:");
    println!(
        "  分散削減率:  {:.2}倍",
        avg_var_uniform / avg_var_importance
    );
    println!(
        "  効率向上:    {:.2}倍",
        avg_var_uniform / avg_var_importance
    );
}
