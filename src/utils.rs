use macroquad::{miniquad::*, prelude::*};
use std::sync::LazyLock;

pub static COMPOSITE_MATERIAL: LazyLock<Material> = LazyLock::new(|| {
    load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: include_str!("composite.glsl"),
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("lightPos", UniformType::Float2)],
            textures: vec!["regular".to_string(), "normal".to_string()],
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
