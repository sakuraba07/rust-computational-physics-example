fn process_data(data: &[f64]) {
    // dataスライス内のすべての要素の平均値を計算する
    let sum: f64 = data.iter().sum();
    let count = data.len();
    if count > 0 {
        println!("Average: {}", sum / count as f64);
    }
}

fn scale(data: &mut [f64], factor: f64) {
    for element in data {
        *element *= factor;
    }
}

fn main() {
    // 3次元の位置ベクトルを表す配列
    let position: [f64; 3] = [1.0, 2.0, 3.0];

    // すべての要素を0.0で初期化
    let velocity: [f64; 3] = [0.0; 3];

    // 要素へのアクセス
    println!("x-coordinate: {}", position[0]);
    println!("Initial velocity: {:?}", velocity);

    // forループによるイテレーション
    for component in &position {
        println!("{}", component);
    }

    let array: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let vector: Vec<f64> = vec![10.0, 20.0, 30.0];

    // 配列全体をスライスとして渡す
    process_data(&array);

    // 配列の一部をスライスとして渡す
    process_data(&array[1..4]); // インデックス1から3までの要素 ([2.0, 3.0, 4.0])

    // Vec全体をスライスとして渡す
    process_data(&vector);

    let mut values = vec![1.0, 2.0, 3.0];
    scale(&mut values, 2.0);
    println!("{:?}", values); // [2.0, 4.0, 6.0]

    // 空のベクタを作成
    let mut time_series: Vec<f64> = Vec::new();

    // vec!マクロで初期値を指定して作成
    let positions = vec![[0.0, 0.0], [1.0, 1.0]];
    println!("Initial positions: {:?}", positions);

    // あらかじめ容量を確保しておくと再割り当てを回避できる
    let results: Vec<f64> = Vec::with_capacity(1000);

    // push: 要素を追加
    time_series.push(0.0);
    time_series.push(0.1);
    time_series.push(0.2);

    // 要素へのアクセス
    println!("Latest data point: {}", time_series[2]);

    // forループによるイテレーション
    for data_point in &time_series {
        println!("{}", data_point);
    }

    println!("Number of data points: {}", time_series.len());
    println!("Results capacity: {}", results.capacity());
}
