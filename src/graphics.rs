use macroquad::{texture::{Texture2D, draw_texture, DrawTextureParams}, window::{Conf, screen_height, screen_width, next_frame}, prelude::{WHITE, BLACK, vec2}};

pub const WINDOW_WIDTH: u16 = 256;
pub const WINDOW_HEIGHT: u16 = 240;

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

