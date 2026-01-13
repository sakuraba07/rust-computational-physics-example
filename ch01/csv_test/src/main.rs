use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.csv")?;

    // ヘッダー行を書き込む
    writeln!(file, "t,x,v")?;

    // 0.0から10.0まで0.1刻みでデータを生成して書き込む
    for i in 0..=100 {
        let t = i as f64 * 0.1;
        let x = t.sin();
        let v = t.cos();
        writeln!(file, "{},{},{}", t, x, v)?;
    }

    println!("output.csv を生成しました");
    Ok(())
}
