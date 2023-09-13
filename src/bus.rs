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

//: Ppu_Data {{{
pub struct PpuData {
    pub nmi_occurred: bool,
     
    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scroll: u16,
    pub addr: u16,
    pub data: u8,
}
// }}} 

//: Bus {{{
pub struct Bus<'a> {
    ram: &'a mut Ram, // 2KB Internal RAM
    cart: &'a Cart, 
    
    pub nmi_signal: bool,
    pub ppu_data: PpuData,

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
                nmi_signal: false,
                ppu_data: PpuData { 
                    nmi_occurred:false, 
                    ctrl: 0, 
                    mask: 0, 
                    status: 0, 
                    oam_addr: 0,
                    oam_data: 0, 
                    scroll: 0, 
                    addr: 0,
                    data: 0 
                }, 
                cpu_debug: false, 
                oam_dma_cpu: false, 
                oam_dma_ppu: false, 
                oam_dma_addr: 0}
    }

    // Interface Functions
    pub fn read(&mut self, mut addr: u16) -> u8 {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x800;
            return self.ram.memory[addr as usize];
        } else if addr < 0x3FFF { // PPU Registers
            addr = (addr % 8) + 0x2000; // Mirrored every 8 bytes
            match addr {
                PPU_CTRL_ADDR => return 0, // Write only
                PPU_MASK_ADDR => return 0, // Write only
                PPU_STATUS_ADDR => {
                    let old_nmi;
                    if self.ppu_data.nmi_occurred {
                        old_nmi = 0xFF; 
                    } else {
                        old_nmi = 0x7F;
                    }

                    self.ppu_data.nmi_occurred = false;

                    self.ppu_data.status & old_nmi
                },
                OAM_ADDR_ADDR => return 0, // Write only
                OAM_DATA_ADDR => return self.ppu_data.oam_data,  
                PPU_SCROLL_ADDR => return 0, // Write only
                PPU_ADDR_ADDR => return 0, // Write only
                PPU_DATA_ADDR => return self.ppu_data.data,
                _ => return 0, // catch all
            }
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
    
    pub fn read_word_little(&mut self, addr: u16) -> u16 {
        let low: u16 = self.read(addr) as u16;

        let high: u16 = self.read(addr + 1) as u16;
        
        return (high << 8) + low;
    }

    pub fn read_word_little_wrap(&mut self, addr: u16) -> u16 {
        let low: u16 = self.read(addr) as u16;

        //let high: u16 = self.read(addr + 1) as u16;
        let low_addr: u8 = (addr as u8).wrapping_add(1);
        let high: u16 = self.read((addr & 0xFF00) + low_addr as u16) as u16;
        
        return (high << 8) + low;
    }

    pub fn write(&mut self, mut addr: u16, value: u8) {
        if addr < 0x2000 { // Internal RAM
            addr = addr % 0x800;
            self.ram.set_memory(addr, value);
        } else if addr < 0x3FFF { // PPU Registers
            addr = (addr % 8) + 0x2000; // Mirrored every 8 bytes
            match addr {
                PPU_CTRL_ADDR => self.ppu_data.ctrl = value,
                PPU_MASK_ADDR => self.ppu_data.mask = value,
                PPU_STATUS_ADDR =>  return, // Read only
                OAM_ADDR_ADDR => self.ppu_data.oam_addr = value,
                OAM_DATA_ADDR => self.ppu_data.oam_data = value,  
                PPU_SCROLL_ADDR => {
                    self.ppu_data.scroll = self.ppu_data.scroll << 8;
                    self.ppu_data.scroll |= value as u16;
                },
                PPU_ADDR_ADDR => {
                    self.ppu_data.addr = self.ppu_data.addr << 8;
                    self.ppu_data.addr |= value as u16;
                },
                PPU_DATA_ADDR => self.ppu_data.data = value,
                _ => return, // catch all
            }
        } else if addr == OAM_DMA_ADDR { // 0x4014
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
