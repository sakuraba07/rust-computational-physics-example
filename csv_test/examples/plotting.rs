use csv;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. CSVファイルからデータを読み込む
    let mut reader = csv::Reader::from_path("output.csv")?;
    let data: Vec<(f64, f64)> = reader
        .records()
        .map(|r| {
            let record = r.unwrap();
            (record[0].parse().unwrap(), record[1].parse().unwrap())
        })
        .collect();

    // 2. 描画バックエンドのセットアップ
    let root = BitMapBackend::new("plot-single.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 3. チャートの構築
    let mut chart = ChartBuilder::on(&root)
        .caption("Position x vs. Time t (from CSV)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, -1.5..1.5)?;

    // 4. メッシュ (軸とグリッド線) の描画
    chart
        .configure_mesh()
        .x_desc("Time t")
        .y_desc("Position x")
        .draw()?;

    // 5. データ系列 (t, x) の描画
    chart.draw_series(LineSeries::new(data, &RED))?;

    // 6. ファイルへの保存
    root.present()?;
    println!("plot-single.png を生成しました");
    Ok(())
}
