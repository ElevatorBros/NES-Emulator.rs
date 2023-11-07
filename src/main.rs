#![allow(dead_code)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use macroquad::window::next_frame;
use nes_emulator::bus::Bus;
use nes_emulator::bus::{WINDOW_HEIGHT, WINDOW_WIDTH};
use nes_emulator::cartridge::Cart;
use nes_emulator::cpu::Cpu;
use nes_emulator::graphics::window_conf;
use nes_emulator::ppu::Ppu;
use nes_emulator::ram::Ram;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let main_cart = match Cart::new(args[1].as_str()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut main_ram = Ram::new(main_cart.header.mirror());
    //let main_cart = match Cart::new("./nestest.nes") {

    let main_bus = Bus::new(&mut main_ram, &main_cart);
    let main_bus_ref = Rc::new(RefCell::new(main_bus));
    let mut main_cpu = Cpu::new(Rc::clone(&main_bus_ref));
    let mut main_ppu = Ppu::new(Rc::clone(&main_bus_ref));

    //main_cpu.pc = 0x0C000; // Nestest.nes
    //main_cpu.cycl = 7;
    //main_cpu.next = 7;

    main_cpu.reset();

    let mut clock = 0;

    loop {
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

        if main_ppu.render_frame {
            let texture = Texture2D::from_rgba8(WINDOW_WIDTH, WINDOW_HEIGHT, &main_ppu.screen);

            //draw_texture(&texture, 0.0, 0.0, WHITE);
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
            next_frame().await;
            main_ppu.render_frame = false;
            //println!("render");
            //println!("vaddr:{}", main_bus.ppu_data.vram_addr);
            //println!("taddr:{}", main_bus.ppu_data.temp_vram_addr);
        }
    }
    //println!("Done");
}
