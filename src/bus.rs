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
    
    pub fn read_word_little(&self, addr: u16) -> u16 {
        let low: u16 = self.read(addr) as u16;

        let high: u16 = self.read(addr + 1) as u16;
        
        return (high << 8) + low;
    }

    pub fn write(&mut self, mut addr: u16, value: u8) {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x0800;
            self.ram.setMemory(addr, value);
        } /* else { // Cartridge space 
            if addr >= 0x8000 {
                addr -= 0x8000;
                self.cart.ROM[addr as usize] = value;
            }
        } */
    }
}
