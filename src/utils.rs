use macroquad::{miniquad::*, prelude::*};
use std::sync::LazyLock;

pub const SCREEN_WIDTH: f32 = 256.0;
pub const SCREEN_HEIGHT: f32 = 144.0;

pub fn create_camera(w: f32, h: f32) -> Camera2D {
    let rt = render_target(w as u32, h as u32);
    rt.texture.set_filter(FilterMode::Nearest);

    Camera2D {
        render_target: Some(rt),
        zoom: Vec2::new(1.0 / w * 2.0, 1.0 / h * 2.0),
        target: vec2(w, h) / 2.0,
        ..Default::default()
    }
}

pub static COMPOSITE_MATERIAL: LazyLock<Material> = LazyLock::new(|| {
    load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: include_str!("composite.glsl"),
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("lightPos", UniformType::Float2)],
            textures: vec!["texture".to_string(), "normal".to_string()],
            ..Default::default()
        },
    )
    .unwrap()
});
pub const DEFAULT_VERTEX_SHADER: &str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";

#[expect(dead_code)]
pub fn get_input_axis() -> Vec2 {
    let mut i = Vec2::ZERO;
    if is_key_down(KeyCode::A) {
        i.x -= 1.0;
    }
    if is_key_down(KeyCode::D) {
        i.x += 1.0;
    }
    if is_key_down(KeyCode::W) {
        i.y -= 1.0;
    }
    if is_key_down(KeyCode::S) {
        i.y += 1.0;
    }
    i.normalize_or_zero()
}
