use crate::Ram;
use crate::Cart;

pub struct Bus<'a> {
    ram: &'a mut Ram, // 2KB Internal RAM
    cart: &'a Cart, 
}


impl<'a> Bus<'a> {
    // Setup Functions
    pub fn new(ram: &'a mut Ram, cart: &'a Cart) -> Self {
        Self { ram, cart}
    }

    // Interface Functions
    pub fn read(&self, mut addr: u16) -> u8 {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x800;
            return self.ram.memory[addr as usize];
        } else { // Cartridge space 
            if addr >= 0x8000 {
                addr -= 0x8000;
                return self.cart.ROM[addr as usize];
            } else {
                return 0;
            }
        }
    }

    pub fn write(&mut self, mut addr: u16, value: u8) {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x800;
            self.ram.setMemory(addr, value);
        } /* else { // Cartridge space 
            if addr >= 0x8000 {
                addr -= 0x8000;
                self.cart.ROM[addr as usize] = value;
            }
        } */
    }
}
