use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // CSVファイルを読み込む
    let csv_filename = "particle_motion.csv";
    let file = File::open(csv_filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    // データを格納するベクター
    let mut time_data = Vec::new();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    let mut vx_data = Vec::new();
    let mut vy_data = Vec::new();

    // CSVデータを読み込む
    for result in rdr.records() {
        let record = result?;
        time_data.push(record[0].parse::<f64>()?);
        x_data.push(record[1].parse::<f64>()?);
        y_data.push(record[2].parse::<f64>()?);
        vx_data.push(record[3].parse::<f64>()?);
        vy_data.push(record[4].parse::<f64>()?);
    }

    println!(
        "CSVファイルから {} データポイントを読み込みました",
        time_data.len()
    );

    // 1. 軌跡のグラフ (x-y平面)
    plot_trajectory(&x_data, &y_data)?;

    // 2. 時間発展のグラフ (位置)
    plot_position_vs_time(&time_data, &x_data, &y_data)?;

    // 3. 時間発展のグラフ (速度)
    plot_velocity_vs_time(&time_data, &vx_data, &vy_data)?;

    println!("グラフを生成しました:");
    println!("  - trajectory.png (軌跡)");
    println!("  - position_vs_time.png (位置の時間発展)");
    println!("  - velocity_vs_time.png (速度の時間発展)");

    Ok(())
}

/// x-y平面上の軌跡をプロット
fn plot_trajectory(x_data: &[f64], y_data: &[f64]) -> Result<(), Box<dyn Error>> {
    let output_filename = "trajectory.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // データの範囲を取得
    let x_min = x_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = 0.0_f64.min(y_data.iter().cloned().fold(f64::INFINITY, f64::min));
    let y_max = y_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // マージンを追加
    let x_margin = (x_max - x_min) * 0.1;
    let y_margin = (y_max - y_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Particle Trajectory", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (x_min - x_margin)..(x_max + x_margin),
            (y_min - y_margin)..(y_max + y_margin),
        )?;

    chart
        .configure_mesh()
        .x_desc("x (m)")
        .y_desc("y (m)")
        .draw()?;

    // 軌跡を描画
    chart
        .draw_series(LineSeries::new(
            x_data.iter().zip(y_data.iter()).map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("Trajectory")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 開始点をマーク
    chart
        .draw_series(std::iter::once(Circle::new(
            (x_data[0], y_data[0]),
            5,
            GREEN.filled(),
        )))?
        .label("Start")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, GREEN.filled()));

    // 終了点をマーク
    let last_idx = x_data.len() - 1;
    chart
        .draw_series(std::iter::once(Circle::new(
            (x_data[last_idx], y_data[last_idx]),
            5,
            RED.filled(),
        )))?
        .label("End")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, RED.filled()));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// 位置の時間発展をプロット
fn plot_position_vs_time(
    time_data: &[f64],
    x_data: &[f64],
    y_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "position_vs_time.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pos_min = x_data
        .iter()
        .chain(y_data.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let pos_max = x_data
        .iter()
        .chain(y_data.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let margin = (pos_max - pos_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Position vs Time", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..t_max, (pos_min - margin)..(pos_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Position (m)")
        .draw()?;

    // x座標のプロット
    chart
        .draw_series(LineSeries::new(
            time_data.iter().zip(x_data.iter()).map(|(t, x)| (*t, *x)),
            &BLUE,
        ))?
        .label("x")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // y座標のプロット
    chart
        .draw_series(LineSeries::new(
            time_data.iter().zip(y_data.iter()).map(|(t, y)| (*t, *y)),
            &RED,
        ))?
        .label("y")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// 速度の時間発展をプロット
fn plot_velocity_vs_time(
    time_data: &[f64],
    vx_data: &[f64],
    vy_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "velocity_vs_time.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let vel_min = vx_data
        .iter()
        .chain(vy_data.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let vel_max = vx_data
        .iter()
        .chain(vy_data.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let margin = (vel_max - vel_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Velocity vs Time", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..t_max, (vel_min - margin)..(vel_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Velocity (m/s)")
        .draw()?;

    // vx のプロット
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(vx_data.iter())
                .map(|(t, vx)| (*t, *vx)),
            &BLUE,
        ))?
        .label("vx")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // vy のプロット
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(vy_data.iter())
                .map(|(t, vy)| (*t, *vy)),
            &RED,
        ))?
        .label("vy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
