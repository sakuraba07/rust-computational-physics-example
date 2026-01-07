use plotters::prelude::*;
use std::error::Error;

// 設定定数
const INPUT_CSV: &str = "output.csv";
const OUTPUT_IMAGE: &str = "plot-single.png";
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. CSVファイルからデータを読み込む
    let mut reader = csv::Reader::from_path(INPUT_CSV)?;
    let data: Vec<(f64, f64)> = reader
        .records()
        .enumerate()
        .map(|(i, r)| {
            let record = r.map_err(|e| format!("行 {} の読み込みエラー: {e}", i + 1))?;
            let t: f64 = record
                .get(0)
                .ok_or_else(|| format!("行 {}: 時刻データが存在しません", i + 1))?
                .parse()
                .map_err(|e| format!("行 {} の時刻パースエラー: {e}", i + 1))?;
            let x: f64 = record
                .get(1)
                .ok_or_else(|| format!("行 {}: 位置データが存在しません", i + 1))?
                .parse()
                .map_err(|e| format!("行 {} の位置パースエラー: {e}", i + 1))?;
            Ok((t, x))
        })
        .collect::<Result<Vec<_>, String>>()?;

    // データが空の場合のチェック
    if data.is_empty() {
        return Err("CSVファイルにデータが含まれていません".into());
    }

    // 軸の範囲を自動計算
    let (t_min, t_max) = data
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), (t, _)| {
            (min.min(*t), max.max(*t))
        });
    let (x_min, x_max) = data
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), (_, x)| {
            (min.min(*x), max.max(*x))
        });

    // マージンを追加
    let t_margin = (t_max - t_min) * 0.05;
    let x_margin = (x_max - x_min) * 0.1;
    let t_range = (t_min - t_margin)..(t_max + t_margin);
    let x_range = (x_min - x_margin)..(x_max + x_margin);

    // 2. 描画バックエンドをセットアップする
    let root = BitMapBackend::new(OUTPUT_IMAGE, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // 3. チャートを構築する
    let mut chart = ChartBuilder::on(&root)
        .caption("Position x vs. Time t (from CSV)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(t_range, x_range)?;

    // 4. メッシュ (軸とグリッド線) を描画する
    chart
        .configure_mesh()
        .x_desc("Time t")
        .y_desc("Position x")
        .draw()?;

    // 5. データ系列 (t, x) を描画する
    chart.draw_series(LineSeries::new(data, &RED))?;

    // 6. ファイルに保存する
    root.present()?;
    println!("{OUTPUT_IMAGE} を生成しました");
    Ok(())
}
