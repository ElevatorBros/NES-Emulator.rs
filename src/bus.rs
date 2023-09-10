// Vim folding 
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ram::Ram;
use crate::cartridge::Cart;
//use crate::Ppu;


pub struct Bus<'a> {
    ram: &'a mut Ram, // 2KB Internal RAM
    //ppu: &'a mut Ppu,
    cart: &'a Cart, 

    pub cpu_debug: bool,
}

//: Bus {{{
impl<'a> Bus<'a> {
    // Setup Functions
    pub fn new(ram: &'a mut Ram, cart: &'a Cart ) -> Self {
        Self { ram, cart, cpu_debug: false }
    }

    // Interface Functions
    pub fn read(&self, mut addr: u16) -> u8 {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x800;
            return self.ram.memory[addr as usize];
        } else if addr < 0x3FFF { // PPU Registers
            //return self.ppu.read(addr);
            return self.ram.memory[addr as usize];
        } else { // Cartridge space 
            if addr < 0x4020 { // stuff
                return 0;
            } else if addr < 0x8000 { // Cart RAM, todo
                return 0;
            } else {
                return self.cart.read(addr);
            }
        }
    }
    
    pub fn read_word_little(&self, addr: u16) -> u16 {
        let low: u16 = self.read(addr) as u16;

        let high: u16 = self.read(addr + 1) as u16;
        
        return (high << 8) + low;
    }

    pub fn read_word_little_wrap(&self, addr: u16) -> u16 {
        let low: u16 = self.read(addr) as u16;

        //let high: u16 = self.read(addr + 1) as u16;
        let low_addr: u8 = (addr as u8).wrapping_add(1);
        let high: u16 = self.read((addr & 0xFF00) + low_addr as u16) as u16;
        
        return (high << 8) + low;
    }

    pub fn write(&mut self, mut addr: u16, value: u8) {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x0800;
            self.ram.set_memory(addr, value);
        } 
        /* else if addr < 0x3FFF { // PPU Registers
            //return self.ppu.write(addr, value);
        //}  else { // Cartridge space 
            if addr >= 0x8000 {
                addr -= 0x8000;
                self.cart.ROM[addr as usize] = value;
            }
        }*/
    }
}
//: }}}
