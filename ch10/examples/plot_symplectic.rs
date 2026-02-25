use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // CSVファイルを読み込む
    let csv_filename = "symplectic.csv";
    let file = File::open(csv_filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    // データを格納するベクター
    let mut time_data = Vec::new();
    let mut position_data = Vec::new();
    let mut velocity_data = Vec::new();
    let mut energy_data = Vec::new();

    // CSVデータを読み込む
    for result in rdr.records() {
        let record = result?;
        time_data.push(record[0].parse::<f64>()?);
        position_data.push(record[1].parse::<f64>()?);
        velocity_data.push(record[2].parse::<f64>()?);
        energy_data.push(record[3].parse::<f64>()?);
    }

    println!(
        "CSVファイルから {} データポイントを読み込みました",
        time_data.len()
    );

    // 1. 位相空間プロット (position-velocity)
    plot_phase_space(&position_data, &velocity_data)?;

    // 2. 位置と速度の時間発展
    plot_position_velocity_vs_time(&time_data, &position_data, &velocity_data)?;

    // 3. エネルギー保存の確認
    plot_energy_vs_time(&time_data, &energy_data)?;

    println!("グラフを生成しました:");
    println!("  - phase_space.png (位相空間)");
    println!("  - position_velocity_vs_time.png (位置・速度の時間発展)");
    println!("  - energy_vs_time.png (エネルギー保存)");

    Ok(())
}

/// 位相空間プロット (position vs velocity)
fn plot_phase_space(position_data: &[f64], velocity_data: &[f64]) -> Result<(), Box<dyn Error>> {
    let output_filename = "phase_space.png";
    let root = BitMapBackend::new(output_filename, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // データの範囲を取得
    let pos_min = position_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let pos_max = position_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let vel_min = velocity_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let vel_max = velocity_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    // マージンを追加
    let pos_margin = (pos_max - pos_min) * 0.1;
    let vel_margin = (vel_max - vel_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Phase Space (Symplectic Integration)", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (pos_min - pos_margin)..(pos_max + pos_margin),
            (vel_min - vel_margin)..(vel_max + vel_margin),
        )?;

    chart
        .configure_mesh()
        .x_desc("Position")
        .y_desc("Velocity")
        .draw()?;

    // 位相空間軌道を描画
    chart
        .draw_series(LineSeries::new(
            position_data
                .iter()
                .zip(velocity_data.iter())
                .map(|(x, v)| (*x, *v)),
            &BLUE,
        ))?
        .label("Trajectory")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 開始点をマーク
    chart
        .draw_series(std::iter::once(Circle::new(
            (position_data[0], velocity_data[0]),
            5,
            GREEN.filled(),
        )))?
        .label("Start")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, GREEN.filled()));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// 位置と速度の時間発展をプロット
fn plot_position_velocity_vs_time(
    time_data: &[f64],
    position_data: &[f64],
    velocity_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "position_velocity_vs_time.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pos_min = position_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let pos_max = position_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let vel_min = velocity_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let vel_max = velocity_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let y_min = pos_min.min(vel_min);
    let y_max = pos_max.max(vel_max);
    let margin = (y_max - y_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Position and Velocity vs Time", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..t_max, (y_min - margin)..(y_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Position / Velocity")
        .draw()?;

    // 位置のプロット
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(position_data.iter())
                .map(|(t, x)| (*t, *x)),
            &BLUE,
        ))?
        .label("Position")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 速度のプロット
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(velocity_data.iter())
                .map(|(t, v)| (*t, *v)),
            &RED,
        ))?
        .label("Velocity")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// エネルギー保存の確認プロット
fn plot_energy_vs_time(time_data: &[f64], energy_data: &[f64]) -> Result<(), Box<dyn Error>> {
    let output_filename = "energy_vs_time.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let e_min = energy_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let e_max = energy_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    // エネルギーの変動範囲を強調するため、小さめのマージン
    let e_center = (e_min + e_max) / 2.0;
    let e_range = (e_max - e_min).max(0.0001); // 最小でも0.0001の範囲を確保
    let margin = e_range * 0.5;

    let mut chart = ChartBuilder::on(&root)
        .caption("Energy Conservation Check", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..t_max, (e_center - margin)..(e_center + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Total Energy")
        .draw()?;

    // エネルギーのプロット
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &BLUE,
        ))?
        .label("Total Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 初期エネルギーの参照線
    if !energy_data.is_empty() {
        let e_initial = energy_data[0];
        chart
            .draw_series(LineSeries::new(
                vec![(0.0, e_initial), (t_max, e_initial)],
                &RED.mix(0.5),
            ))?
            .label("Initial Energy")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.mix(0.5)));
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
