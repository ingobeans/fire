use macroquad::prelude::*;

use crate::utils::*;

/// Handles rendering of textures that are lit with normal maps
pub struct RenderingEngine {
    /// Camera that textures are rendered to.
    texture_camera: Camera2D,
    /// Camera that textures' corresponding normal maps are rendered to.
    normal_camera: Camera2D,
    /// Camera that the composited & lighted scene is rendered to.
    composite_camera: Camera2D,
}
impl RenderingEngine {
    pub fn new() -> Self {
        let composite_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
        let mut texture_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
        let mut normal_camera = create_camera(SCREEN_WIDTH, SCREEN_HEIGHT);
        let target = vec2(2.0 * 16.0, 2.0 * 16.0);
        texture_camera.target = target;
        normal_camera.target = target;
        Self {
            texture_camera,
            normal_camera,
            composite_camera,
        }
    }
    pub fn use_texture_camera(&self) {
        set_camera(&self.texture_camera);
    }
    pub fn use_normal_camera(&self) {
        set_camera(&self.normal_camera);
    }
    /// Returns the rendered output of the composite camera. `render` should have been called first.
    pub fn get_rendered_texture(&self) -> &Texture2D {
        &self
            .composite_camera
            .render_target
            .as_ref()
            .unwrap()
            .texture
    }
    /// Composites texture and normal cameras and calculates lighting
    pub fn render(&mut self, light_pos: Vec2) {
        set_camera(&self.composite_camera);
        clear_background(BLACK);
        COMPOSITE_MATERIAL.set_texture(
            "texture",
            self.texture_camera
                .render_target
                .as_ref()
                .unwrap()
                .texture
                .clone(),
        );
        COMPOSITE_MATERIAL.set_texture(
            "normal",
            self.normal_camera
                .render_target
                .as_ref()
                .unwrap()
                .texture
                .clone(),
        );

        COMPOSITE_MATERIAL.set_uniform("lightPos", light_pos / vec2(SCREEN_WIDTH, SCREEN_HEIGHT));
        gl_use_material(&COMPOSITE_MATERIAL);
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, WHITE);
        gl_use_default_material();
    }
}
