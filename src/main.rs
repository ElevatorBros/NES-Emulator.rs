//pub mod ram;
//pub mod cartrige;
//pub mod bus;
//pub mod cpu;
#![allow(dead_code)]
#![allow(unused_variables)]

use NES_Emulator::Ram;
use NES_Emulator::Cart;
use NES_Emulator::Ppu;
use NES_Emulator::Bus;
use NES_Emulator::Cpu;


fn main() {
    let mut main_ram = Ram::new();
    let mut main_cart = match Cart::new("./nestest.nes") {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let mut main_ppu = Ppu::new();

    /*main_cart.ROM[0x00] = 0xA9;
    main_cart.ROM[0x01] = 0x07;
    main_cart.ROM[0x02] = 0x90;
    main_cart.ROM[0x03] = 0xFC;
    */

    //let mut buffer = [0u8; 8]; // the buffer can be reused!
    //for i in 0..16384 {
    //    main_cart.rom[(i+(0x0C000 - 0x08000 - 0x10)) as usize] = main_cart.rom[i as usize];
    //}

    let mut main_bus = Bus::new(&mut main_ram, &mut main_ppu, &main_cart);

    let mut main_cpu = Cpu::new(&mut main_bus);
    

    main_cpu.pc = 0x0C000;
    main_cpu.cycl = 7;
    main_cpu.next = 7;
    for _i in 0..26554 {
        main_cpu.clock();
    }
    println!("Done");
}
