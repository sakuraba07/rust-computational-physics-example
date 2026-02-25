use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // CSVファイルを読み込む
    let csv_filename = "kepler.csv";
    let file = File::open(csv_filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    // データを格納するベクター
    let mut time_data = Vec::new();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    let mut vx_data = Vec::new();
    let mut vy_data = Vec::new();
    let mut energy_data = Vec::new();
    let mut angular_momentum_data = Vec::new();

    // CSVデータを読み込む
    for result in rdr.records() {
        let record = result?;
        time_data.push(record[0].parse::<f64>()?);
        x_data.push(record[1].parse::<f64>()?);
        y_data.push(record[2].parse::<f64>()?);
        vx_data.push(record[3].parse::<f64>()?);
        vy_data.push(record[4].parse::<f64>()?);
        energy_data.push(record[5].parse::<f64>()?);
        angular_momentum_data.push(record[6].parse::<f64>()?);
    }

    println!(
        "CSVファイルから {} データポイントを読み込みました",
        time_data.len()
    );

    // 1. 軌道プロット (x-y plane)
    plot_orbit(&x_data, &y_data)?;

    // 2. エネルギーと角運動量の保存確認
    plot_conserved_quantities(&time_data, &energy_data, &angular_momentum_data)?;

    // 3. 動径距離の時間発展
    plot_radial_distance(&time_data, &x_data, &y_data)?;

    println!("グラフを生成しました:");
    println!("  - orbit.png (軌道)");
    println!("  - conserved_quantities.png (保存量)");
    println!("  - radial_distance.png (動径距離)");

    Ok(())
}

/// 軌道をプロット
fn plot_orbit(x_data: &[f64], y_data: &[f64]) -> Result<(), Box<dyn Error>> {
    let output_filename = "orbit.png";
    let root = BitMapBackend::new(output_filename, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // データの範囲を取得
    let x_min = x_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = y_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // 正方形のプロット領域を作るため、両軸で最大の範囲を使用
    let max_range = (x_max - x_min).max(y_max - y_min);
    let x_center = (x_max + x_min) / 2.0;
    let y_center = (y_max + y_min) / 2.0;
    let margin = max_range * 0.1;

    let range = (max_range / 2.0) + margin;

    let mut chart = ChartBuilder::on(&root)
        .caption("Kepler Orbit", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (x_center - range)..(x_center + range),
            (y_center - range)..(y_center + range),
        )?;

    chart
        .configure_mesh()
        .x_desc("x (AU)")
        .y_desc("y (AU)")
        .draw()?;

    // 軌道を描画
    chart
        .draw_series(LineSeries::new(
            x_data.iter().zip(y_data.iter()).map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("Orbit")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 太陽の位置（原点）をマーク
    chart
        .draw_series(std::iter::once(Circle::new((0.0, 0.0), 8, YELLOW.filled())))?
        .label("Sun")
        .legend(|(x, y)| Circle::new((x + 10, y), 8, YELLOW.filled()));

    // 開始点をマーク
    chart
        .draw_series(std::iter::once(Circle::new(
            (x_data[0], y_data[0]),
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

/// エネルギーと角運動量の保存をプロット
fn plot_conserved_quantities(
    time_data: &[f64],
    energy_data: &[f64],
    angular_momentum_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "conserved_quantities.png";
    let root = BitMapBackend::new(output_filename, (800, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let areas = root.split_evenly((2, 1));

    // エネルギーのプロット
    plot_energy(&areas[0], time_data, energy_data)?;

    // 角運動量のプロット
    plot_angular_momentum(&areas[1], time_data, angular_momentum_data)?;

    root.present()?;
    Ok(())
}

fn plot_energy(
    drawing_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
    time_data: &[f64],
    energy_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let e_min = energy_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let e_max = energy_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let e_center = (e_min + e_max) / 2.0;
    let e_range = (e_max - e_min).max(1e-10);
    let margin = e_range * 0.5;

    let mut chart = ChartBuilder::on(drawing_area)
        .caption("Energy Conservation", ("sans-serif", 25))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..t_max, (e_center - margin)..(e_center + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (yr)")
        .y_desc("Total Energy")
        .draw()?;

    chart.draw_series(LineSeries::new(
        time_data
            .iter()
            .zip(energy_data.iter())
            .map(|(t, e)| (*t, *e)),
        &BLUE,
    ))?;

    // 初期エネルギーの参照線
    if !energy_data.is_empty() {
        let e_initial = energy_data[0];
        chart.draw_series(LineSeries::new(
            vec![(0.0, e_initial), (t_max, e_initial)],
            &RED.mix(0.5),
        ))?;
    }

    Ok(())
}

fn plot_angular_momentum(
    drawing_area: &DrawingArea<BitMapBackend, plotters::coord::Shift>,
    time_data: &[f64],
    angular_momentum_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let l_min = angular_momentum_data
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let l_max = angular_momentum_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let l_center = (l_min + l_max) / 2.0;
    let l_range = (l_max - l_min).max(1e-10);
    let margin = l_range * 0.5;

    let mut chart = ChartBuilder::on(drawing_area)
        .caption("Angular Momentum Conservation", ("sans-serif", 25))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..t_max, (l_center - margin)..(l_center + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (yr)")
        .y_desc("Angular Momentum")
        .draw()?;

    chart.draw_series(LineSeries::new(
        time_data
            .iter()
            .zip(angular_momentum_data.iter())
            .map(|(t, l)| (*t, *l)),
        &BLUE,
    ))?;

    // 初期角運動量の参照線
    if !angular_momentum_data.is_empty() {
        let l_initial = angular_momentum_data[0];
        chart.draw_series(LineSeries::new(
            vec![(0.0, l_initial), (t_max, l_initial)],
            &RED.mix(0.5),
        ))?;
    }

    Ok(())
}

/// 動径距離の時間発展をプロット
fn plot_radial_distance(
    time_data: &[f64],
    x_data: &[f64],
    y_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "radial_distance.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // 動径距離を計算
    let r_data: Vec<f64> = x_data
        .iter()
        .zip(y_data.iter())
        .map(|(x, y)| (x * x + y * y).sqrt())
        .collect();

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let r_min = r_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let r_max = r_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let margin = (r_max - r_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Radial Distance vs Time", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..t_max, (r_min - margin)..(r_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time (yr)")
        .y_desc("Radial Distance (AU)")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            time_data.iter().zip(r_data.iter()).map(|(t, r)| (*t, *r)),
            &BLUE,
        ))?
        .label("r(t)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
