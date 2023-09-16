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

#[macroquad::main(window_conf)]
async fn main() {
    let mut main_ram = Ram::new();
    let main_cart = match Cart::new("./nestest.nes") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let mut main_bus = Bus::new(&mut main_ram, &main_cart);
    let mut main_cpu = Cpu::new();
    let mut main_ppu = Ppu::new();

    main_cpu.pc = 0x0C000;
    main_cpu.cycl = 7;
    main_cpu.next = 7;

    let mut clock = 0;

    loop {
        if clock % 12 == 0 {
            main_cpu.clock(&mut main_bus);
        }
        if clock % 4 == 0 {
            main_ppu.clock(&mut main_bus);
        }

        // 3840 = 16 * 12 * 5 * 4 (NTSC & PAL clock divides)
        if clock == 3840 {
            clock = 0;
        } else {
            clock += 1;
        }

        if main_ppu.render_frame {
            let texture = Texture2D::from_rgba8(WINDOW_WIDTH, WINDOW_HEIGHT, &main_ppu.screen);

            draw_texture(&texture, 0.0, 0.0, WHITE);
            next_frame().await;
            main_ppu.render_frame = false;
        }
    }
    //println!("Done");
}
