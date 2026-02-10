use ndarray::Array1;
use num_complex::Complex64;
use rustfft::FftPlanner;

fn main() {
    let n = 8;
    // ndarrayでデータを作成
    let mut data =
        Array1::<Complex64>::from_iter((0..n).map(|i| Complex64::new(i as f64 + 1.0, 0.0)));

    // FFTのプランを作成
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);

    // ndarrayの内部バッファをスライスとして取り出し、FFTを実行
    // processメソッドはインプレース（破壊的）に計算を行う
    fft.process(data.as_slice_mut().expect("Array must be contiguous"));

    println!("FFT 結果:");
    for (i, val) in data.iter().enumerate() {
        println!("{}: {:.3} + {:.3}i", i, val.re, val.im);
    }
}
