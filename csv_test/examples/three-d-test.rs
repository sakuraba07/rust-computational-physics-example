use three_d::*;

// ウィンドウ設定
const WINDOW_TITLE: &str = "three-d: Cube";
const WINDOW_MAX_SIZE: (u32, u32) = (1280, 720);

// カメラ設定
const CAMERA_POSITION: (f32, f32, f32) = (0.0, 2.0, 4.0);
const CAMERA_TARGET: (f32, f32, f32) = (0.0, 0.0, 0.0);
const CAMERA_UP: (f32, f32, f32) = (0.0, 1.0, 0.0);
const CAMERA_FOV_DEGREES: f32 = 45.0;
const CAMERA_NEAR: f32 = 0.1;
const CAMERA_FAR: f32 = 100.0;

// カメラコントロール設定
const ORBIT_MIN_DISTANCE: f32 = 1.0;
const ORBIT_MAX_DISTANCE: f32 = 10.0;

// 立方体の色 (RGBA: 青色)
const CUBE_COLOR: Srgba = Srgba::new(0, 128, 255, 255);

// 光源設定
const DIRECTIONAL_LIGHT_INTENSITY: f32 = 2.0;
const DIRECTIONAL_LIGHT_DIRECTION: (f32, f32, f32) = (0.0, -1.0, -1.0);
const AMBIENT_LIGHT_INTENSITY: f32 = 0.4;

// 回転速度 (ラジアン/秒)
const ROTATION_SPEED: f32 = 1.0;

// 背景色 (RGBA)
const BACKGROUND_COLOR: (f32, f32, f32, f32) = (0.1, 0.1, 0.1, 1.0);

fn main() {
    // ウィンドウの作成
    let window = Window::new(WindowSettings {
        title: WINDOW_TITLE.to_string(),
        max_size: Some(WINDOW_MAX_SIZE),
        ..Default::default()
    })
    .expect("ウィンドウの作成に失敗しました");

    let context = window.gl();

    // カメラの設定
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(CAMERA_POSITION.0, CAMERA_POSITION.1, CAMERA_POSITION.2),
        vec3(CAMERA_TARGET.0, CAMERA_TARGET.1, CAMERA_TARGET.2),
        vec3(CAMERA_UP.0, CAMERA_UP.1, CAMERA_UP.2),
        degrees(CAMERA_FOV_DEGREES),
        CAMERA_NEAR,
        CAMERA_FAR,
    );

    // カメラコントローラーの設定（マウスで視点操作可能）
    let mut control = OrbitControl::new(camera.target(), ORBIT_MIN_DISTANCE, ORBIT_MAX_DISTANCE);

    // 立方体の作成
    let mut cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: CUBE_COLOR,
                ..Default::default()
            },
        ),
    );

    // 光源の設定
    let light = DirectionalLight::new(
        &context,
        DIRECTIONAL_LIGHT_INTENSITY,
        Srgba::WHITE,
        vec3(
            DIRECTIONAL_LIGHT_DIRECTION.0,
            DIRECTIONAL_LIGHT_DIRECTION.1,
            DIRECTIONAL_LIGHT_DIRECTION.2,
        ),
    );
    let ambient = AmbientLight::new(&context, AMBIENT_LIGHT_INTENSITY, Srgba::WHITE);

    // 回転角度（ラジアン）
    let mut angle: f32 = 0.0;

    // メインループ
    window.render_loop(move |mut frame_input| {
        // ビューポートの更新
        camera.set_viewport(frame_input.viewport);

        // マウス操作でカメラを更新
        control.handle_events(&mut camera, &mut frame_input.events);

        // フレームレートに依存しない回転
        // elapsed_time はミリ秒単位なので秒に変換
        let delta_time = frame_input.elapsed_time as f32 / 1000.0;
        angle += ROTATION_SPEED * delta_time;
        cube.set_transformation(Mat4::from_angle_y(radians(angle)));

        // フレームをレンダリング
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(
                BACKGROUND_COLOR.0,
                BACKGROUND_COLOR.1,
                BACKGROUND_COLOR.2,
                BACKGROUND_COLOR.3,
                1.0,
            ))
            .render(&camera, &cube, &[&light, &ambient]);

        FrameOutput::default()
    });
}
