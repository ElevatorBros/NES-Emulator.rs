#![allow(dead_code)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use macroquad::window::next_frame;
use nes_emulator::bus::Bus;
use nes_emulator::bus::{WINDOW_HEIGHT, WINDOW_WIDTH};
use nes_emulator::cartridge::Cart;
use nes_emulator::cpu::Cpu;
use nes_emulator::graphics::window_conf;
use nes_emulator::input::Input;
use nes_emulator::ppu::Ppu;
use nes_emulator::ram::Ram;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Catridge loaded, currently the path is provided as the first argument
    let main_cart = match Cart::new(args[1].as_str()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    // Random Access Memory (both on nes and cart)
    let mut main_ram = Ram::new(main_cart.header.mirror());

    // User input handler
    let mut input = Input::new();

    // Bus which links everything together
    let main_bus = Bus::new(&mut main_ram, &main_cart, &mut input);
    let main_bus_ref = Rc::new(RefCell::new(main_bus));

    // Central Processing Unit (6502)
    let mut main_cpu = Cpu::new(Rc::clone(&main_bus_ref));

    // Pixel Processing Unit (2C02)
    let mut main_ppu = Ppu::new(Rc::clone(&main_bus_ref));

    // Move CPU into reset start to start the program
    main_cpu.reset();

    // Track clock for timings
    let mut clock = 0;

    // Flag to pause the game
    let mut pause = false;

    // Enable to view pattern table while playing
    let pattern_table_debug_veiw = false;

    loop {
        if !pause {
            use std::time::Instant;
            let now = Instant::now();
            while !main_ppu.render_frame {
                // Clock cpu and ppu at their respective clock divides
                // Currently NTSC
                if clock % 12 == 0 {
                    main_cpu.clock();
                }
                if clock % 4 == 0 {
                    main_ppu.clock();
                }

                // 3840 = 16 * 12 * 5 * 4 (NTSC & PAL clock divides)
                if clock == 3840 {
                    clock = 0;
                } else {
                    clock += 1;
                }
            }
            let elapsed = now.elapsed();
        }

        // If the ppu has finished drawing to the frame buffer, allow macroquad to render a frame
        if main_ppu.render_frame || pause {
            // Pausing logic
            if is_key_pressed(KeyCode::Space) {
                pause = !pause;
                if pause {
                    println!("Pause");
                } else {
                    println!("Play");
                }
            }

            let delta = get_frame_time();

            // Drawe from main_ppu.screen
            let texture = Texture2D::from_rgba8(WINDOW_WIDTH, WINDOW_HEIGHT, &main_ppu.screen);
            draw_texture_ex(
                &texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: WINDOW_WIDTH as f32 * 3.0,
                        y: WINDOW_HEIGHT as f32 * 3.0,
                    }),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );

            // pattern table debug start
            if pattern_table_debug_veiw {
                main_ppu.fill_pattern_tables();
                let plane_left = Texture2D::from_rgba8(128, 128, &main_ppu.pattern_table_left);
                let plane_right = Texture2D::from_rgba8(128, 128, &main_ppu.pattern_table_right);

                draw_texture_ex(
                    &plane_left,
                    WINDOW_WIDTH as f32 * 3.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: (128 * 3) as f32,
                            y: (128 * 3) as f32,
                        }),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );

                draw_texture_ex(
                    &plane_right,
                    WINDOW_WIDTH as f32 * 3.0 + (128 * 3) as f32,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2 {
                            x: (128 * 3) as f32,
                            y: (128 * 3) as f32,
                        }),
                        source: None,
                        rotation: 0.0,
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                );
            }
            // pattern table debug end

            // Let macroquad render
            next_frame().await;

            // Unset render ready flag
            main_ppu.render_frame = false;
        }
    }
}
