# csv_test

Rust 計算物理学チュートリアル 第1部 第1章 3節のサンプルプロジェクトです。CSV データの生成、2D プロット、3D レンダリングの例を含みます。

## 必要なツール

- [Rust](https://www.rust-lang.org/ja/tools/install) (Edition 2024)
- [FFmpeg](https://ffmpeg.org/)（動画出力を行う場合のみ）

## ファイル構成

| ファイル                     | 説明                                          |
| ---------------------------- | --------------------------------------------- |
| `src/main.rs`                | CSV ファイル (`output.csv`) を生成            |
| `examples/plotting.rs`       | CSV データを読み込み、単一系列のグラフを描画  |
| `examples/plotting-multi.rs` | CSV データを読み込み、複数系列のグラフを描画  |
| `examples/three-d-test.rs`   | 3D 立方体をウィンドウに表示（マウス操作可能） |
| `examples/three-d-video.rs`  | 3D アニメーションを動画ファイルに出力         |

## 実行方法

### 1. CSV データの生成

まず、プロット用の CSV データを生成します。

```sh
cargo run
```

`output.csv` が生成されます。

### 2. 2D グラフの描画

単一系列（位置 x のみ）のグラフを描画します。

```sh
cargo run --example plotting
```

`plot-single.png` が生成されます。

複数系列（位置 x と速度 v）のグラフを描画します。

```sh
cargo run --example plotting-multi
```

`plot-multi.png` が生成されます。

### 3. 3D レンダリング

インタラクティブな 3D ビューアを起動します。マウスで視点を操作できます。

```sh
cargo run --example three-d-test
```

ウィンドウを閉じるか、`Esc` キーで終了します。

### 4. 3D 動画の出力

> **注意**: この例を実行するには FFmpeg が必要です。

3D アニメーションを動画ファイルに出力します。

```sh
cargo run --example three-d-video
```

`three-d-video.webm` が生成されます。

## ライセンス

CC0-1.0
