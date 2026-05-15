//! Pixel canvas - software rasterizer for pixel-art style rendering.
//!
//! Used by pipedream for isometric pixel rendering.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Plugin for pixel canvas rendering.
pub struct PixelCanvasPlugin;

impl Plugin for PixelCanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_pixel_canvas_system);
    }
}

/// Component marking an entity as a pixel canvas.
#[derive(Component)]
pub struct PixelCanvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA
    pub dirty: bool,
    pub image_handle: Handle<Image>,
}

impl PixelCanvas {
    pub fn new(width: u32, height: u32, image_handle: Handle<Image>) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            pixels: vec![0; size],
            dirty: true,
            image_handle,
        }
    }

    /// Clear canvas to a color.
    pub fn clear(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for chunk in self.pixels.chunks_mut(4) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = a;
        }
        self.dirty = true;
    }

    /// Set a pixel at (x, y).
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        self.pixels[idx] = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = a;
        self.dirty = true;
    }

    /// Draw a filled rectangle.
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, r, g, b, a);
            }
        }
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8, a: u8) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                self.set_pixel(x as u32, y as u32, r, g, b, a);
            }
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

/// System that updates the texture from pixel canvas data.
fn update_pixel_canvas_system(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<&mut PixelCanvas>,
) {
    for mut canvas in query.iter_mut() {
        if canvas.dirty {
            if let Some(image) = images.get_mut(&canvas.image_handle) {
                image.data.clone_from(&canvas.pixels);
            }
            canvas.dirty = false;
        }
    }
}

/// Create a new image handle for a pixel canvas.
pub fn create_canvas_image(images: &mut Assets<Image>, width: u32, height: u32) -> Handle<Image> {
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );
    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
        | bevy::render::render_resource::TextureUsages::COPY_DST;
    images.add(image)
}
