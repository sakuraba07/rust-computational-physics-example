use ndarray::arr1;
use sprs::TriMat;

fn main() {
    let rows = 4;
    let cols = 4;
    // トリプレット形式 (Triplet Format) で初期化
    let mut triplet = TriMat::new((rows, cols));

    // (row, col, value) を追加
    triplet.add_triplet(0, 0, 1.0);
    triplet.add_triplet(0, 3, 2.0);
    triplet.add_triplet(1, 1, 3.0);
    triplet.add_triplet(2, 0, 4.0);
    triplet.add_triplet(2, 2, 5.0);
    triplet.add_triplet(3, 3, 6.0);

    // CSR形式に変換
    let a_csr = triplet.to_csr::<usize>();

    println!("CSR Matrix:\n{:?}", a_csr);

    // 行列ベクトル積
    let x = arr1(&[1.0, 1.0, 1.0, 1.0]);

    // sprsの行列とndarrayのベクトルの積
    // 結果も ndarray のベクトルになる
    let y = &a_csr * &x;

    println!("y = A * x = {}", y);
    // 期待値:
    // row0: 1*1 + 0 + 0 + 2*1 = 3
    // row1: 3*1 = 3
    // row2: 4*1 + 5*1 = 9
    // row3: 6*1 = 6
    // y = [3, 3, 9, 6]
}
