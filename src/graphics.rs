use macroquad::{texture::{Texture2D, draw_texture_ex, DrawTextureParams}, window::Conf, prelude::{WHITE, vec2}};

const WINDOW_WIDTH: u16 = 256;
const WINDOW_HEIGHT: u16 = 256;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

pub fn put(bytes: &[u8]) {
    let texture = Texture2D::from_rgba8(WINDOW_WIDTH, WINDOW_HEIGHT, bytes);
    draw_texture_ex(
        &texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
            ..Default::default()
        },
    );
}
