use plotters::prelude::*;
use std::error::Error;

// 定数定義
const INPUT_CSV: &str = "output.csv";
const OUTPUT_IMAGE: &str = "plot-multi.png";
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

// データポイント構造体
struct DataPoint {
    t: f64,
    x: f64,
    v: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // CSVからデータを読み込み、構造化データとしてパース
    let mut reader = csv::Reader::from_path(INPUT_CSV)?;
    let data: Vec<DataPoint> = reader
        .records()
        .enumerate()
        .map(|(i, result)| {
            let record = result.map_err(|e| format!("行 {} の読み込みエラー: {e}", i + 2))?;
            Ok(DataPoint {
                t: record[0]
                    .parse()
                    .map_err(|e| format!("行 {} の時刻パースエラー: {e}", i + 2))?,
                x: record[1]
                    .parse()
                    .map_err(|e| format!("行 {} の位置パースエラー: {e}", i + 2))?,
                v: record[2]
                    .parse()
                    .map_err(|e| format!("行 {} の速度パースエラー: {e}", i + 2))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    // 軸範囲の自動計算
    let (t_min, t_max) = data.iter().fold((f64::MAX, f64::MIN), |(min, max), d| {
        (min.min(d.t), max.max(d.t))
    });
    let (val_min, val_max) = data.iter().fold((f64::MAX, f64::MIN), |(min, max), d| {
        (min.min(d.x).min(d.v), max.max(d.x).max(d.v))
    });

    // マージンを追加
    let t_margin = (t_max - t_min) * 0.05;
    let val_margin = (val_max - val_min) * 0.1;
    let t_range = (t_min - t_margin)..(t_max + t_margin);
    let val_range = (val_min - val_margin)..(val_max + val_margin);

    let root = BitMapBackend::new(OUTPUT_IMAGE, (IMAGE_WIDTH, IMAGE_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Position and Velocity (from CSV)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(t_range, val_range)?;

    chart
        .configure_mesh()
        .x_desc("Time t")
        .y_desc("Value")
        .draw()?;

    // 位置 x のデータ系列
    let pos_series = LineSeries::new(data.iter().map(|d| (d.t, d.x)), &RED);

    // 速度 v のデータ系列
    let vel_series = LineSeries::new(data.iter().map(|d| (d.t, d.v)), &BLUE);

    // 系列を描画し、凡例を設定
    chart
        .draw_series(pos_series)?
        .label("Position x")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(vel_series)?
        .label("Velocity v")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 凡例の描画
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("{OUTPUT_IMAGE} を生成しました");
    Ok(())
}
