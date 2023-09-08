//pub mod ram;
//pub mod cartrige;
//pub mod bus;
//pub mod cpu;
#![allow(dead_code)]
#![allow(unused_variables)]

use NES_Emulator::ram::Ram;
use NES_Emulator::cartrige::Cart;
use NES_Emulator::ppu::Ppu;
use NES_Emulator::bus::Bus;
use NES_Emulator::cpu::Cpu;
use std::sync::mpsc;

fn main() {
    let mut main_ram = Ram::new();
    let main_cart = match Cart::new("./nestest.nes") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };


    let (sender, receiver) = mpsc::channel();
    let mut main_ppu = Ppu::new(sender, receiver);

    /*main_cart.ROM[0x00] = 0xA9;
    main_cart.ROM[0x01] = 0x07;
    main_cart.ROM[0x02] = 0x90;
    main_cart.ROM[0x03] = 0xFC;
    */

    //let mut buffer = [0u8; 8]; // the buffer can be reused!
    //for i in 0..16384 {
    //    main_cart.rom[(i+(0x0C000 - 0x08000 - 0x10)) as usize] = main_cart.rom[i as usize];
    //}

    let mut main_bus = Bus::new(&mut main_ram, &main_cart);
    let mut main_cpu = Cpu::new();

    main_cpu.pc = 0x0C000;
    main_cpu.cycl = 7;
    main_cpu.next = 7;


    for _i in 0..100000 {
        main_ppu.clock(&mut main_bus);
    }

    loop {
        //main_cpu.clock(&mut main_bus);
        //main_ppu.clock(&mut main_bus); 
    }

    println!("Done");
}
