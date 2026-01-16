use plotters::prelude::*;
use std::error::Error;

// 設定定数
const INPUT_CSV: &str = "diff_error.csv";
const OUTPUT_IMAGE: &str = "diff_error.png";
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
            let h: f64 = record
                .get(0)
                .ok_or_else(|| format!("行 {}: データが存在しません", i + 1))?
                .parse()
                .map_err(|e| format!("行 {} のパースエラー: {e}", i + 1))?;
            let error: f64 = record
                .get(1)
                .ok_or_else(|| format!("行 {}: データが存在しません", i + 1))?
                .parse()
                .map_err(|e| format!("行 {} のパースエラー: {e}", i + 1))?;
            Ok((h, error))
        })
        .collect::<Result<Vec<_>, String>>()?;

    // データが空の場合のチェック
    if data.is_empty() {
        return Err("CSVファイルにデータが含まれていません".into());
    }

    // 軸の範囲を自動計算
    let (h_min, h_max) = data
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), (h, _)| {
            (min.min(*h), max.max(*h))
        });
    let (error_min, error_max) = data
        .iter()
        .fold((f64::MAX, f64::MIN), |(min, max), (_, error)| {
            (min.min(*error), max.max(*error))
        });

    // 対数スケール用の範囲を設定（正の値のみを使用）
    let h_min_log = h_min.max(1e-10);
    let h_max_log = h_max.max(h_min_log * 10.0);
    let error_min_log = error_min.max(1e-10);
    let error_max_log = error_max.max(error_min_log * 10.0);

    // 2. 描画バックエンドをセットアップする
    let root = BitMapBackend::new(OUTPUT_IMAGE, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    // 3. チャートを構築する（両対数スケール）
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Differentiation Error vs. Step Size (Log-Log)",
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (h_min_log..h_max_log).log_scale(),
            (error_min_log..error_max_log).log_scale(),
        )?;

    // 4. メッシュ (軸とグリッド線) を描画する
    chart
        .configure_mesh()
        .x_desc("Step size h")
        .y_desc("Error")
        .x_label_formatter(&|x| format!("{:.0e}", x))
        .y_label_formatter(&|y| format!("{:.0e}", y))
        .draw()?;

    // 5. データ系列 (t, x) を描画する
    chart.draw_series(LineSeries::new(data, &RED))?;

    // 6. ファイルに保存する
    root.present()?;
    println!("{OUTPUT_IMAGE} を生成しました");
    Ok(())
}
