use ndarray::{Array, Array1};
use num_complex::Complex64;
use plotters::prelude::*;
use rustfft::FftPlanner;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 1024;
    let fs = 1000.0; // サンプリング周波数 1000Hz
    let dt = 1.0 / fs;

    // 1. 信号の生成(50Hz + 120Hz)
    let t = Array::range(0.0, n as f64 * dt, dt);
    let mut signal = t.mapv(|ti| {
        let val = 1.0 * (2.0 * PI * 50.0 * ti).sin() + 0.5 * (2.0 * PI * 120.0 * ti).sin();
        Complex64::new(val, 0.0)
    });

    // 2. 窓関数の適用(Hanning窓)
    let window = Array1::from_shape_fn(n, |i| {
        0.5 * (1.0 - (2.0 * PI * i as f64 / (n as f64 - 1.0)).cos())
    });
    // 要素ごとの積
    signal *= &window.mapv(|w| Complex64::new(w, 0.0));

    // 3. FFTの実行
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(signal.as_slice_mut().unwrap());

    // 4. 結果の解析(パワースペクトルの計算)
    let freqs: Vec<f64> = (0..n / 2).map(|k| k as f64 * fs / n as f64).collect();
    let powers: Vec<f64> = signal.iter().take(n / 2).map(|c| c.norm_sqr()).collect();

    // 5. 可視化
    let root = BitMapBackend::new("spectrum.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Power Spectrum", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0.0..fs / 2.0,
            0.0..*powers
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Frequency [Hz]")
        .y_desc("Power")
        .draw()?;

    chart.draw_series(LineSeries::new(
        freqs.into_iter().zip(powers.into_iter()),
        &RED,
    ))?;

    root.present()?;
    println!("spectrum.png を生成しました。");

    Ok(())
}
