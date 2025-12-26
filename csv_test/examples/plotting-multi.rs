use csv;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // CSVからデータを読み込む
    let mut reader = csv::Reader::from_path("output.csv")?;
    let records: Vec<_> = reader.records().collect();

    let root = BitMapBackend::new("plot-multi.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Position and Velocity (from CSV)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, -1.5..1.5)?;
    chart
        .configure_mesh()
        .x_desc("Time t")
        .y_desc("Value")
        .draw()?;

    // 位置 x のデータ系列
    let pos_series = LineSeries::new(
        records.iter().map(|r| {
            let record = r.as_ref().unwrap();
            (record[0].parse().unwrap(), record[1].parse().unwrap())
        }),
        &RED,
    );

    // 速度 v のデータ系列
    let vel_series = LineSeries::new(
        records.iter().map(|r| {
            let record = r.as_ref().unwrap();
            (record[0].parse().unwrap(), record[2].parse().unwrap())
        }),
        &BLUE,
    );

    // 系列を描画し、凡例を設定
    chart
        .draw_series(pos_series)?
        .label("Position x")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(vel_series)?
        .label("Velocity v")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 凡例の描画
    chart
        .configure_series_labels()
        // 凡例の位置を右上に指定
        .position(SeriesLabelPosition::UpperRight)
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("plot-multi.png を生成しました");
    Ok(())
}
