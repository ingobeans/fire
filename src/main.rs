use macroquad::{miniquad::window::screen_size, prelude::*};

const SCREEN_WIDTH: f32 = 256.0;
const SCREEN_HEIGHT: f32 = 144.0;

pub fn create_camera(w: f32, h: f32) -> Camera2D {
    let rt = render_target(w as u32, h as u32);
    rt.texture.set_filter(FilterMode::Nearest);

    Camera2D {
        render_target: Some(rt),
        zoom: Vec2::new(1.0 / w * 2.0, 1.0 / h * 2.0),
        // target: vec2(w, h) / 2.0,
        ..Default::default()
    }
}

#[macroquad::main("fire")]
async fn main() {
    let texture = Texture2D::from_file_with_format(include_bytes!("../brick.png"), None);
    let camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    loop {
        let (w, h) = screen_size();
        let scale_factor = (w / SCREEN_WIDTH).min(h / SCREEN_HEIGHT).floor();
        set_camera(&camera);
        for i in 0..4 {
            for j in 0..4 {
                draw_texture(&texture, 16.0 * i as f32, 16.0 * j as f32, WHITE);
            }
        }

        set_default_camera();

        clear_background(BLACK);
        draw_texture_ex(
            &camera.render_target.as_ref().unwrap().texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT) * scale_factor),
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
