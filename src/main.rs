use macroquad::{miniquad::window::screen_size, prelude::*};

mod utils;
use utils::*;

const SCREEN_WIDTH: f32 = 256.0;
const SCREEN_HEIGHT: f32 = 144.0;

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
    let light = Texture2D::from_file_with_format(include_bytes!("../light.png"), None);

    let mut light_pos;

    let mut regular_pass = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut normal_pass = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    let target = vec2(2.0 * 16.0, 2.0 * 16.0);
    regular_pass.target = target;
    normal_pass.target = target;
    let composite = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
    loop {
        let (w, h) = screen_size();
        let scale_factor = (w / SCREEN_WIDTH).min(h / SCREEN_HEIGHT).floor();

        set_camera(&regular_pass);
        for i in 0..4 {
            for j in 0..4 {
                draw_texture(&texture, 16.0 * i as f32, 16.0 * j as f32, WHITE);
            }
        }
        set_camera(&normal_pass);
        for i in 0..4 {
            for j in 0..4 {
                draw_texture(&normal, 16.0 * i as f32, 16.0 * j as f32, WHITE);
            }
        }

        set_camera(&composite);
        clear_background(BLACK);
        COMPOSITE_MATERIAL.set_texture(
            "regular",
            regular_pass.render_target.as_ref().unwrap().texture.clone(),
        );
        COMPOSITE_MATERIAL.set_texture(
            "normal",
            normal_pass.render_target.as_ref().unwrap().texture.clone(),
        );
        // light_pos += get_input_axis() * get_frame_time() * 64.0;

        light_pos = Vec2::from(mouse_position()) / scale_factor;

        COMPOSITE_MATERIAL.set_uniform("lightPos", light_pos / vec2(SCREEN_WIDTH, SCREEN_HEIGHT));
        gl_use_material(&COMPOSITE_MATERIAL);
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, WHITE);
        gl_use_default_material();

        draw_texture(&light, light_pos.x - 8.0, light_pos.y - 8.0, WHITE);

        set_default_camera();
        clear_background(BLACK);
        let size = vec2(SCREEN_WIDTH, SCREEN_HEIGHT) * scale_factor;
        draw_texture_ex(
            &composite.render_target.as_ref().unwrap().texture,
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
