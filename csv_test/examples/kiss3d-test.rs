use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{UnitQuaternion, Vector3};

#[tokio::main]
async fn main() {
    // ウィンドウの作成
    let mut window = Window::new("kiss3d: Cube");

    // 立方体の追加
    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(0.0, 0.5, 1.0); // 青色

    // 光源の設定
    window.set_light(Light::StickToCamera);

    // メインループ (無限ループ)
    // ウィンドウが閉じられるとプログラムが終了します。
    loop {
        // Y軸を中心に少し回転させる
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.01);
        cube.prepend_to_local_rotation(&rotation);

        // フレームを非同期でレンダリング
        window.render().await;
    }
}
