//pub mod ram;
//pub mod cartrige;
//pub mod bus;
//pub mod cpu;

use NES_Emulator::Ram;
use NES_Emulator::Cart;
use NES_Emulator::Bus;
use NES_Emulator::Cpu;


use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut main_ram = Ram::new();
    let mut main_cart = Cart::new();

    /*main_cart.ROM[0x00] = 0xA9;
    main_cart.ROM[0x01] = 0x07;
    main_cart.ROM[0x02] = 0x90;
    main_cart.ROM[0x03] = 0xFC;
    */

    //let mut buffer = [0u8; 8]; // the buffer can be reused!
    // Create a path to the desired file
    let path = Path::new("./nestest.nes");

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    file.read(&mut main_cart.ROM);
    //println!("len:0x{:X} = {}", file.metadata().unwrap().len(), file.metadata().unwrap().len());
    for i in 0..16384 {
        main_cart.ROM[(i+(0x0C000 - 0x08000 - 0x10)) as usize] = main_cart.ROM[i as usize];
    }

    let mut main_bus = Bus::new(&mut main_ram, &main_cart);

    let mut main_cpu = Cpu::new(&mut main_bus);

    main_cpu.pc = 0x0C000;
    main_cpu.cycl = 7;
    main_cpu.next = 7;
    for _i in 0..10000 {
        main_cpu.clock();
    }
}
