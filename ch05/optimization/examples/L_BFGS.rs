use argmin::core::{CostFunction, Error, Executor, Gradient};
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin::solver::quasinewton::LBFGS;
use ndarray::{Array1, array};

struct Rosenbrock {}

// 1. 目的関数の定義: f(x)
impl CostFunction for Rosenbrock {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        let (x, y) = (p[0], p[1]);

        Ok((1.0 - x).powi(2) + 100.0 * (y - x.powi(2)).powi(2))
    }
}

// 2. 勾配 (1階微分) の定義: grad f(x)
impl Gradient for Rosenbrock {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        let (x, y) = (p[0], p[1]);

        let gx = -2.0 * (1.0 - x) - 400.0 * x * (y - x.powi(2));
        let gy = 200.0 * (y - x.powi(2));

        Ok(array![gx, gy])
    }
}

fn main() {
    let cost = Rosenbrock {};
    let init_param = array![-1.2, 1.0];

    // L-BFGSソルバーの設定
    // 準ニュートン法では、更新の方向を決めた後に「どれだけ進むか」を決める
    // 行探索 (Line Search) アルゴリズムが必須となります。
    let linesearch = MoreThuenteLineSearch::new();

    // L-BFGS::new(行探索, 記憶する履歴の数)
    // 履歴の数(m)は通常 3~10 程度で十分な性能を発揮します。
    let solver = LBFGS::new(linesearch, 7);

    // 3. Executorによる実行
    let res = Executor::new(cost, solver)
        .configure(|state| {
            state
                .param(init_param) // 初期値
                .max_iters(100) // 最大反復回数
                .target_cost(1e-10) // 目標値に達したら終了
        })
        .run()
        .expect("Optimization failed");

    // 結果の表示 (argmin 0.11では OptimizationResult が返る)
    println!("Result: {}", res);
}
