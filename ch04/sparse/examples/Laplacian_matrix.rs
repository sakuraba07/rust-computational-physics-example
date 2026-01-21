use sprs::TriMat;

fn main() {
    let n = 10; // グリッド点数
    let mut triplet = TriMat::new((n, n));

    for i in 0..n {
        // 対角成分 -2.0
        triplet.add_triplet(i, i, -2.0);

        // 隣接成分 1.0
        if i > 0 {
            triplet.add_triplet(i, i - 1, 1.0);
        }
        if i < n - 1 {
            triplet.add_triplet(i, i + 1, 1.0);
        }
    }

    let laplacian = triplet.to_csr::<usize>();
    // これでサイズが大きくてもメモリ効率よく保持できる

    println!("Laplacian matrix (CSR format):\n{:?}", laplacian);
}
