use plotters::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // CSVファイルを読み込む
    let csv_filename = "molecular_dynamics.csv";
    let file = File::open(csv_filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    // データを格納するベクター
    let mut step_data = Vec::new();
    let mut time_data = Vec::new();
    let mut potential_energy_data = Vec::new();
    let mut kinetic_energy_data = Vec::new();
    let mut total_energy_data = Vec::new();

    // CSVデータを読み込む
    for result in rdr.records() {
        let record = result?;
        step_data.push(record[0].parse::<usize>()?);
        time_data.push(record[1].parse::<f64>()?);
        potential_energy_data.push(record[2].parse::<f64>()?);
        kinetic_energy_data.push(record[3].parse::<f64>()?);
        total_energy_data.push(record[4].parse::<f64>()?);
    }

    println!(
        "CSVファイルから {} データポイントを読み込みました",
        time_data.len()
    );

    // 1. エネルギー成分の時間発展
    plot_energy_components(&time_data, &potential_energy_data, &kinetic_energy_data)?;

    // 2. 全エネルギーの保存確認
    plot_total_energy(&time_data, &total_energy_data)?;

    // 3. すべてのエネルギーをまとめて表示
    plot_all_energies(
        &time_data,
        &potential_energy_data,
        &kinetic_energy_data,
        &total_energy_data,
    )?;

    println!("グラフを生成しました:");
    println!("  - md_energy_components.png (エネルギー成分)");
    println!("  - md_total_energy.png (全エネルギー)");
    println!("  - md_all_energies.png (全エネルギーまとめ)");

    Ok(())
}

/// ポテンシャルエネルギーと運動エネルギーの時間発展をプロット
fn plot_energy_components(
    time_data: &[f64],
    potential_energy_data: &[f64],
    kinetic_energy_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "md_energy_components.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pot_min = potential_energy_data
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let pot_max = potential_energy_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let kin_min = kinetic_energy_data
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let kin_max = kinetic_energy_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let y_min = pot_min.min(kin_min);
    let y_max = pot_max.max(kin_max);
    let margin = (y_max - y_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Energy Components vs Time", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..t_max, (y_min - margin)..(y_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Energy")
        .draw()?;

    // ポテンシャルエネルギー
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(potential_energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &BLUE,
        ))?
        .label("Potential Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 運動エネルギー
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(kinetic_energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &RED,
        ))?
        .label("Kinetic Energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

/// 全エネルギーの保存をプロット
fn plot_total_energy(time_data: &[f64], total_energy_data: &[f64]) -> Result<(), Box<dyn Error>> {
    let output_filename = "md_total_energy.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let e_min = total_energy_data
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let e_max = total_energy_data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let e_center = (e_min + e_max) / 2.0;
    let e_range = (e_max - e_min).max(1e-10);
    let margin = e_range * 0.5;

    let mut chart = ChartBuilder::on(&root)
        .caption("Total Energy Conservation", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..t_max, (e_center - margin)..(e_center + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Total Energy")
        .draw()?;

    // 全エネルギー
    chart.draw_series(LineSeries::new(
        time_data
            .iter()
            .zip(total_energy_data.iter())
            .map(|(t, e)| (*t, *e)),
        &BLUE,
    ))?;

    // 初期エネルギーの参照線
    if !total_energy_data.is_empty() {
        let e_initial = total_energy_data[0];
        chart.draw_series(LineSeries::new(
            vec![(0.0, e_initial), (t_max, e_initial)],
            &RED.mix(0.5),
        ))?;
    }

    root.present()?;
    Ok(())
}

/// すべてのエネルギーをまとめてプロット
fn plot_all_energies(
    time_data: &[f64],
    potential_energy_data: &[f64],
    kinetic_energy_data: &[f64],
    total_energy_data: &[f64],
) -> Result<(), Box<dyn Error>> {
    let output_filename = "md_all_energies.png";
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_max = time_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // 全データの範囲を取得
    let all_values: Vec<f64> = potential_energy_data
        .iter()
        .chain(kinetic_energy_data.iter())
        .chain(total_energy_data.iter())
        .cloned()
        .collect();

    let y_min = all_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = all_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let margin = (y_max - y_min) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("All Energy Components", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..t_max, (y_min - margin)..(y_max + margin))?;

    chart
        .configure_mesh()
        .x_desc("Time")
        .y_desc("Energy")
        .draw()?;

    // ポテンシャルエネルギー
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(potential_energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &BLUE,
        ))?
        .label("Potential")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // 運動エネルギー
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(kinetic_energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &RED,
        ))?
        .label("Kinetic")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // 全エネルギー
    chart
        .draw_series(LineSeries::new(
            time_data
                .iter()
                .zip(total_energy_data.iter())
                .map(|(t, e)| (*t, *e)),
            &GREEN,
        ))?
        .label("Total")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
