use ndarray::{Array, Array1, Array2, Axis, arr1, arr2, s};

fn main() {
    // スライスからベクトルを作成
    let a: Array1<f64> = Array::from(vec![1.0, 2.0, 3.0]);
    println!("a = {}", a);

    // arr1 関数を使った、より簡潔な作成方法
    let b = arr1(&[4.0, 5.0, 6.0]);
    println!("b = {}", b);

    // 0から9までの連番を持つベクトルを作成
    let c: Array1<f64> = Array::range(0.0, 10.0, 1.0);
    println!("c = {}", c);

    // すべての要素が0.0のベクトルを作成（次元を指定）
    let zeros = Array1::<f64>::zeros(5);
    println!("zeros = {}", zeros);

    // ベクトルの加算
    println!("a + b = {}", &a + &b);

    // ベクトルの減算
    println!("a - b = {}", &a - &b);

    // スカラー倍
    println!("a * 2.0 = {}", &a * 2.0);

    // 要素ごとの積
    println!("a * b (element-wise) = {}", &a * &b);

    // 内積 (dot product)
    let dot_product = a.dot(&b);
    println!("a . b = {}", dot_product);

    // arr2 関数で行列を作成
    // 各内部配列が「行」に対応する
    let m = arr2(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
    println!("m =\n{}", m);

    // from_shape_vecで行列を作成
    // (行数, 列数)のタプルと、データを格納したVecを渡す
    let shape = (3, 2);
    let data = vec![1, 2, 3, 4, 5, 6];
    let m_from_vec = Array2::from_shape_vec(shape, data).expect("Incompatible shape");
    println!("m_from_vec =\n{}", m_from_vec);

    // 3x3のゼロ行列
    let zeros = Array2::<f64>::zeros((3, 3));
    println!("zeros =\n{}", zeros);

    // 2x2の単位行列
    let eye = Array2::<f64>::eye(2);
    println!("eye =\n{}", eye);

    let m1 = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    let m2 = arr2(&[[5.0, 6.0], [7.0, 8.0]]);

    // 行列の加算
    println!("m1 + m2 =\n{}", &m1 + &m2);

    // 行列のスカラー倍
    println!("m1 * 3.0 =\n{}", &m1 * 3.0);

    // 行列積
    let matrix_product = m1.dot(&m2);
    println!("m1 * m2 (matrix product) =\n{}", matrix_product);

    // 行列とベクトルの積
    let v = arr1(&[10.0, 20.0]);
    let matrix_vector_product = m1.dot(&v);
    println!("m1 * v =\n{}", matrix_vector_product);

    let m = arr2(&[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    println!("m =\n{}", m);

    // 0番目の行を取得
    let row0 = m.slice(s![0, ..]);
    println!("Row 0: {}", row0); // [1, 2, 3, 4]

    // 1番目の列を取得
    let col1 = m.slice(s![.., 1]);
    println!("Column 1: {}", col1); // [2, 6, 10]

    // 部分行列を取得
    // 0-1行目、1-2列目を抜き出す
    let sub_matrix = m.slice(s![0..2, 1..3]);
    println!("Sub-matrix (0..2, 1..3) =\n{}", sub_matrix);
    // [[2, 3],
    //  [6, 7]]

    // スライスは元のデータのビュー（参照）
    // スライスを介して元のデータを変更することも可能
    let mut m_mut = m.clone();
    let mut sub_view = m_mut.slice_mut(s![0..2, 1..3]);
    sub_view.fill(0); // 部分行列を0で埋める
    println!("Modified m_mut =\n{}", m_mut);

    let m = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
    let v = arr1(&[10.0, 20.0]);

    // 行列の各行にベクトルvを加算する
    // vは (2,) -> (1, 2) にブロードキャストされ、
    // それがさらに (2, 2) に引き伸ばされてmと加算される
    let result_row = &m + &v;
    println!("m + v (row-wise broadcast) =\n{}", result_row);

    // 行列の各列にベクトルを加算する場合
    // ベクトルを列ベクトルとして扱うために次元を追加する必要がある
    // v.view().insert_axis(Axis(1)) は vの形状を (2,) から (2, 1) に変える
    // view() を使うことで v の所有権を保持したままビューを操作できる
    let result_col = &m + &v.view().insert_axis(Axis(1));
    println!("m + v (column-wise broadcast) =\n{}", result_col);
}
