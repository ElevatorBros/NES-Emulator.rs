// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use macroquad::prelude::*;

pub struct Input {
    // Current Button State
    // Layout:
    // 0 - A
    // 1 - B
    // 2 - Select
    // 3 - Start
    // 4 - Up
    // 5 - Down
    // 6 - Left
    // 7 - Right
    pub joypad_one: u8,

    // false: latch input
    // true: read input continually
    pub read_latch: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            joypad_one: 0,
            read_latch: false,
        }
    }

    pub fn set_a_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00000001;
        } else {
            self.joypad_one &= 0b11111110;
        }
    }

    pub fn set_b_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00000010;
        } else {
            self.joypad_one &= 0b11111101;
        }
    }

    pub fn set_select_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00000100;
        } else {
            self.joypad_one &= 0b11111011;
        }
    }

    pub fn set_start_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00001000;
        } else {
            self.joypad_one &= 0b11110111;
        }
    }

    pub fn set_up_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00010000;
        } else {
            self.joypad_one &= 0b11101111;
        }
    }

    pub fn set_down_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b00100000;
        } else {
            self.joypad_one &= 0b11011111;
        }
    }

    pub fn set_left_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b01000000;
        } else {
            self.joypad_one &= 0b10111111;
        }
    }

    pub fn set_right_input(&mut self, val: bool) {
        if val {
            self.joypad_one |= 0b10000000;
        } else {
            self.joypad_one &= 0b01111111;
        }
    }

    pub fn update_input(&mut self) {
        if self.read_latch {
            self.set_a_input(is_key_down(macroquad::prelude::KeyCode::A));
            self.set_b_input(is_key_down(macroquad::prelude::KeyCode::O));
            self.set_select_input(is_key_down(macroquad::prelude::KeyCode::E));
            self.set_start_input(is_key_down(macroquad::prelude::KeyCode::U));
            self.set_up_input(is_key_down(macroquad::prelude::KeyCode::Up));
            self.set_down_input(is_key_down(macroquad::prelude::KeyCode::Down));
            self.set_left_input(is_key_down(macroquad::prelude::KeyCode::Left));
            self.set_right_input(is_key_down(macroquad::prelude::KeyCode::Right));
        }
    }

    pub fn set_latch(&mut self, val: bool) {
        self.read_latch = val;
    }

    pub fn read_and_shift_joypad_one(&mut self) -> u8 {
        let bit = self.joypad_one & 1;
        self.joypad_one >>= 1;
        bit
    }
}
