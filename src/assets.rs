use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::prelude::*;
use std::sync::LazyLock;

pub static ASSETS: LazyLock<Assets> = LazyLock::new(|| Assets::new());

pub struct Assets {
    pub tileset: LightedSpritesheet,
}
impl Assets {
    fn new() -> Self {
        Self {
            tileset: LightedSpritesheet::new(include_bytes!("../assets/tileset.ase")),
        }
    }
}

#[expect(dead_code)]
fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct LightedSpritesheet {
    pub texture: Spritesheet,
    pub normal: Spritesheet,
}
impl LightedSpritesheet {
    fn new(bytes: &[u8]) -> Self {
        let img = AsepriteFile::read(bytes).unwrap();
        let get_layer = |layer| {
            let img = img.layer(layer).frame(0).image();

            let new = Image {
                width: img.width() as u16,
                height: img.height() as u16,
                bytes: img.as_bytes().to_vec(),
            };
            let texture = Texture2D::from_image(&new);
            texture.set_filter(FilterMode::Nearest);
            Spritesheet::new(texture, 16.0)
        };
        Self {
            texture: get_layer(0),
            normal: get_layer(1),
        }
    }
}

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    #[expect(dead_code)]
    /// Same as `draw_tile`, except centered
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<(DrawTextureParams, Color)>,
    ) {
        self.draw_tile(
            screen_x - self.sprite_size / 2.0,
            screen_y - self.sprite_size / 2.0,
            tile_x,
            tile_y,
            params,
        );
    }
    /// Draws a single tile from the spritesheet
    pub fn draw_tile(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<(DrawTextureParams, Color)>,
    ) {
        let (mut p, color) = params.unwrap_or((DrawTextureParams::default(), WHITE));
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(&self.texture, screen_x, screen_y, color, p);
    }
}
