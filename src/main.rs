//pub mod ram;
//pub mod cartrige;
//pub mod bus;
//pub mod cpu;

use NES_Emulator::Ram;
use NES_Emulator::Cart;
use NES_Emulator::Bus;
use NES_Emulator::Cpu;

fn main() {
    let mut mainRam = Ram::new();
    let mut mainCart = Cart::new();
    mainCart.ROM[0x00] = 0xA9;
    mainCart.ROM[0x01] = 0x07;
    mainCart.ROM[0x02] = 0x90;
    mainCart.ROM[0x03] = 0xFC;

    let mut mainBus = Bus::new(&mut mainRam, &mainCart);

    let mut mainCpu = Cpu::new(&mut mainBus);

    for i in 0..20 {
        mainCpu.clock();
    }
}
