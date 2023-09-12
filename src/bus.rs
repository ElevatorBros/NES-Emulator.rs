// Vim folding 
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::ram::Ram;
use crate::cartridge::Cart;

pub const WINDOW_WIDTH: u16 = 256;
pub const WINDOW_HEIGHT: u16 = 240;


pub const PPU_CTRL_ADDR: u16 = 0x2000;
pub const PPU_MASK_ADDR: u16 = 0x2001;
pub const PPU_STATUS_ADDR: u16 = 0x2002;
pub const OAM_ADDR_ADDR: u16 = 0x2003;
pub const OAM_DATA_ADDR: u16 = 0x2004;
pub const PPU_SCROLL_ADDR: u16 = 0x2005;
pub const PPU_ADDR_ADDR: u16 = 0x2006;
pub const PPU_DATA_ADDR: u16 = 0x2007;
pub const OAM_DMA_ADDR: u16 = 0x4014;
//: Bus {{{
pub struct Bus<'a> {
    ram: &'a mut Ram, // 2KB Internal RAM
    cart: &'a Cart, 

    pub ppu_current_scroll: u16,
    pub ppu_current_addr: u16,

    pub cpu_debug: bool,
    pub oam_dma_cpu: bool,
    pub oam_dma_ppu: bool,
    pub oam_dma_addr: u16,
}
//}}}

//: Bus Functions {{{
impl<'a> Bus<'a> {
    // Setup Functions
    pub fn new(ram: &'a mut Ram, cart: &'a Cart ) -> Self {
        Self {  ram, 
                cart, 
                ppu_current_scroll: 0, 
                ppu_current_addr: 0, 
                cpu_debug: false, 
                oam_dma_cpu: false, 
                oam_dma_ppu: false, 
                oam_dma_addr: 0}
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
        } else if addr == PPU_SCROLL_ADDR {
            self.ppu_current_scroll = self.ppu_current_scroll << 8;
            self.ppu_current_scroll |= value as u16;
        } else if addr == PPU_ADDR_ADDR {
            self.ppu_current_addr = self.ppu_current_addr << 8;
            self.ppu_current_addr |= value as u16;
        } else if addr == OAM_DMA_ADDR {
            self.oam_dma_cpu = true;
            self.oam_dma_ppu = true;
            self.oam_dma_addr = (value as u16) << 8;
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
