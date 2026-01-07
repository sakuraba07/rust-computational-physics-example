use std::io::Write;
use std::process::{Child, Command, Stdio};
use three_d::*;

// ウィンドウ設定
const WINDOW_TITLE: &str = "three-d: Video Export";
const WINDOW_SIZE: (u32, u32) = (1280, 720);

// カメラ設定
const CAMERA_POSITION: (f32, f32, f32) = (0.0, 2.0, 4.0);
const CAMERA_TARGET: (f32, f32, f32) = (0.0, -0.3, 0.0);
const CAMERA_UP: (f32, f32, f32) = (0.0, 1.0, 0.0);
const CAMERA_FOV_DEGREES: f32 = 45.0;
const CAMERA_NEAR: f32 = 0.1;
const CAMERA_FAR: f32 = 100.0;

// 立方体の色 (RGBA: 青色)
const CUBE_COLOR: Srgba = Srgba::new(0, 128, 255, 255);

// 光源設定
const DIRECTIONAL_LIGHT_INTENSITY: f32 = 2.0;
const DIRECTIONAL_LIGHT_DIRECTION: (f32, f32, f32) = (0.0, -1.0, -1.0);
const AMBIENT_LIGHT_INTENSITY: f32 = 0.4;

// 背景色 (RGBA)
const BACKGROUND_COLOR: (f32, f32, f32, f32) = (0.1, 0.1, 0.1, 1.0);

// 動画出力設定
const OUTPUT_FILE: &str = "three-d-video.webm";
const FPS: u32 = 60;
const DURATION_SECONDS: f32 = 5.0;
const ROTATION_SPEED: f32 = 1.0; // ラジアン/秒

/// FFmpegのエンコーダーをラップする構造体
struct FfmpegEncoder {
    child: Child,
    width: u32,
    height: u32,
}

impl FfmpegEncoder {
    fn new(width: u32, height: u32, fps: u32, output_file: &str) -> Self {
        let child = Command::new("ffmpeg")
            .args([
                "-y", // 出力ファイルを上書き
                "-f",
                "rawvideo", // 入力形式: 生のビデオデータ
                "-pixel_format",
                "rgba", // ピクセルフォーマット
                "-video_size",
                &format!("{}x{}", width, height), // 解像度
                "-framerate",
                &fps.to_string(), // フレームレート
                "-i",
                "-", // 標準入力から読み込み
                "-c:v",
                "libvpx-vp9", // VP9コーデック
                "-pix_fmt",
                "yuva420p", // 出力ピクセルフォーマット
                "-b:v",
                "2M",        // ビットレート
                output_file, // 出力ファイル
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect(
                "FFmpegの起動に失敗しました。FFmpegがインストールされていることを確認してください。",
            );

        Self {
            child,
            width,
            height,
        }
    }

    fn write_frame(&mut self, pixels: &[[u8; 4]]) -> Result<(), std::io::Error> {
        // ピクセルデータをバイト列に変換（反転なし - three-dは既に正しい向き）
        let mut data = Vec::with_capacity((self.width * self.height * 4) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = pixels[(y * self.width + x) as usize];
                data.extend_from_slice(&pixel);
            }
        }

        if let Some(ref mut stdin) = self.child.stdin {
            stdin.write_all(&data)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "FFmpeg stdin is closed",
            ))
        }
    }

    fn finish(&mut self) {
        // stdinをドロップしてFFmpegに終了を通知
        self.child.stdin.take();
        // FFmpegの終了を待つ
        let _ = self.child.wait();
    }
}

fn main() {
    // ウィンドウの作成
    let window = Window::new(WindowSettings {
        title: WINDOW_TITLE.to_string(),
        max_size: Some(WINDOW_SIZE),
        ..Default::default()
    })
    .expect("ウィンドウの作成に失敗しました");

    let context = window.gl();

    // カメラの設定（初期ビューポートはウィンドウサイズで設定、後で更新される）
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(CAMERA_POSITION.0, CAMERA_POSITION.1, CAMERA_POSITION.2),
        vec3(CAMERA_TARGET.0, CAMERA_TARGET.1, CAMERA_TARGET.2),
        vec3(CAMERA_UP.0, CAMERA_UP.1, CAMERA_UP.2),
        degrees(CAMERA_FOV_DEGREES),
        CAMERA_NEAR,
        CAMERA_FAR,
    );

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

    // 総フレーム数
    let total_frames = (FPS as f32 * DURATION_SECONDS) as u32;
    let delta_time = 1.0 / FPS as f32;

    // フレームカウンタ
    let mut frame_count: u32 = 0;
    let mut angle: f32 = 0.0;
    let mut finished = false;

    // FFmpegエンコーダー（最初のフレームで実際のビューポートサイズを取得して初期化）
    let mut encoder: Option<FfmpegEncoder> = None;

    // メインループ
    window.render_loop(move |frame_input| {
        if finished {
            return FrameOutput {
                exit: true,
                ..Default::default()
            };
        }

        // 実際のビューポートサイズを取得
        let viewport = frame_input.viewport;
        let actual_width = viewport.width;
        let actual_height = viewport.height;

        // 最初のフレームでFFmpegを初期化
        if encoder.is_none() {
            println!(
                "実際のビューポートサイズ: {}x{} (Retinaスケール適用後)",
                actual_width, actual_height
            );
            println!(
                "動画出力開始: {}フレーム ({:.1}秒, {}FPS)",
                total_frames, DURATION_SECONDS, FPS
            );
            println!("出力ファイル: {}", OUTPUT_FILE);
            encoder = Some(FfmpegEncoder::new(
                actual_width,
                actual_height,
                FPS,
                OUTPUT_FILE,
            ));
        }

        if frame_count >= total_frames {
            println!("\n出力完了: {}", OUTPUT_FILE);
            if let Some(ref mut enc) = encoder {
                enc.finish();
            }
            finished = true;
            return FrameOutput {
                exit: true,
                ..Default::default()
            };
        }

        // ビューポートの更新
        camera.set_viewport(viewport);

        // 立方体の回転
        cube.set_transformation(Mat4::from_angle_y(radians(angle)));

        // 画面にレンダリング
        let screen = frame_input.screen();
        screen
            .clear(ClearState::color_and_depth(
                BACKGROUND_COLOR.0,
                BACKGROUND_COLOR.1,
                BACKGROUND_COLOR.2,
                BACKGROUND_COLOR.3,
                1.0,
            ))
            .render(&camera, &cube, &[&light, &ambient]);

        // 画面からピクセルデータを読み取り
        let pixels: Vec<[u8; 4]> = screen.read_color();

        // FFmpegにフレームデータを送信
        if let Some(ref mut enc) = encoder
            && enc.write_frame(&pixels).is_err()
        {
            eprintln!("\nFFmpegへの書き込みに失敗しました");
            enc.finish();
            finished = true;
            return FrameOutput {
                exit: true,
                ..Default::default()
            };
        }

        // 進捗表示
        let progress = (frame_count + 1) as f32 / total_frames as f32 * 100.0;
        print!(
            "\rフレーム {}/{} ({:.1}%)",
            frame_count + 1,
            total_frames,
            progress
        );
        std::io::stdout().flush().ok();

        // 次のフレームへ
        angle += ROTATION_SPEED * delta_time;
        frame_count += 1;

        FrameOutput::default()
    });
}
