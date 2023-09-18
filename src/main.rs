#![allow(dead_code)]
#![allow(unused_variables)]

use nes_emulator::ram::Ram;
use nes_emulator::cartridge::Cart;
use nes_emulator::ppu::Ppu;
use nes_emulator::bus::Bus;
use nes_emulator::bus::{WINDOW_WIDTH, WINDOW_HEIGHT};
use nes_emulator::cpu::Cpu;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::WindowBuilder,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut main_ram = Ram::new();
    let main_cart = Cart::new("./nestest.nes")?;
    let mut main_bus = Bus::new(&mut main_ram, &main_cart);
    let mut main_cpu = Cpu::new();
    let mut main_ppu = Ppu::new();

    main_cpu.pc = 0x0C000;
    main_cpu.cycl = 7;
    main_cpu.next = 7; 

    let mut clock = 0;

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        let scaled_size = LogicalSize::new(WINDOW_WIDTH as f64 * 3.0, WINDOW_HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("NES Emulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, surface_texture)?
    };

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
            pixels.frame_mut().copy_from_slice(&main_ppu.screen);
            pixels.render()?;
            window.request_redraw();
            main_ppu.render_frame = false;
        }
    }
    // println!("Done");
}
