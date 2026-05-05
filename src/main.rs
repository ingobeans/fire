use macroquad::{miniquad::window::screen_size, prelude::*};

mod graphics;
mod utils;
use graphics::*;
use utils::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "fire".to_string(),
        window_width: SCREEN_WIDTH as i32 * 3,
        window_height: SCREEN_HEIGHT as i32 * 3,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let texture = Texture2D::from_file_with_format(include_bytes!("../brick.png"), None);
    let normal = Texture2D::from_file_with_format(include_bytes!("../normal.png"), None);

    let mut fullscreen = false;

    let mut renderer = RenderingEngine::new();

    loop {
        let (w, h) = screen_size();
        let scale_factor = (w / SCREEN_WIDTH).min(h / SCREEN_HEIGHT).floor();

        if is_key_pressed(KeyCode::Enter) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
            if !fullscreen {
                // reset to default screen size
                request_new_screen_size(SCREEN_WIDTH * 3.0, SCREEN_HEIGHT * 3.0);
            }
        }

        renderer.use_texture_camera();
        for i in 0..4 {
            for j in 0..4 {
                draw_texture(&texture, 16.0 * i as f32, 16.0 * j as f32, WHITE);
            }
        }
        renderer.use_normal_camera();
        for i in 0..4 {
            for j in 0..4 {
                draw_texture(&normal, 16.0 * i as f32, 16.0 * j as f32, WHITE);
            }
        }

        renderer.render(Vec2::from(mouse_position()) / scale_factor);

        set_default_camera();
        clear_background(BLACK);
        let size = vec2(SCREEN_WIDTH, SCREEN_HEIGHT) * scale_factor;
        draw_texture_ex(
            &renderer.get_rendered_texture(),
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
