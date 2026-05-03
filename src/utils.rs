use macroquad::{miniquad::*, prelude::*};
use std::sync::LazyLock;

pub static COMPOSITE_MATERIAL: LazyLock<Material> = LazyLock::new(|| {
    load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: include_str!("composite.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("y", UniformType::Float1),
                UniformDesc::new("height", UniformType::Float1),
                UniformDesc::new("maxTowerHeight", UniformType::Float1),
            ],
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
