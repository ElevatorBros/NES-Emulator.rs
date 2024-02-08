// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::cartridge::Cart;
use crate::input::Input;
use crate::ppu::PpuData;
use crate::ram::Ram;

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
pub const JOYPAD_ONE_ADDR: u16 = 0x4016;

//: Bus {{{
pub struct Bus<'a> {
    ram: &'a mut Ram, 
    cart: &'a Cart,
    input: &'a mut Input,

    pub nmi_signal: bool,  // Non Maskable Interrupt Signal, flag is here 
                           // because it goes from PPU to CPU 
    pub ppu_data: PpuData, // PPU Data that must be accessed by other 
                           // parts of the code. Defined in ppu.rs
    pub cpu_debug: bool,   // Flag to display assembled instructions as they execute 
    pub oam_dma_cpu: bool, // Flag for CPU to halt during OAM DMA 
    pub oam_dma_ppu: bool, // Flag for PPU to perform OAM DMA
    pub oam_dma_addr: u16, // Address of the OAM DMA
}
//}}}

//: Bus Functions {{{
impl<'a> Bus<'a> {
    // Setup Functions
    pub fn new(ram: &'a mut Ram, cart: &'a Cart, input: &'a mut Input) -> Self {
        Self {
            ram,
            cart,
            input,
            nmi_signal: false,
            ppu_data: PpuData {
                nmi_occurred: false,
                ctrl: 0,
                mask: 0,
                status: 0,
                oam_addr: 0,
                scroll_latch: false,
                addr_latch: false,
                data: 0,
                data_buffer: 0,
                fine_x_scroll: 0,
                vram_addr: 0,
                temp_vram_addr: 0,

                oam: [0; 0x100],
            },
            cpu_debug: false,
            oam_dma_cpu: false,
            oam_dma_ppu: false,
            oam_dma_addr: 0,
        }
    }

    // Interface Functions
    // Read a byte
    pub fn read(&mut self, mut addr: u16, debug: bool) -> u8 {
        if addr < 0x2000 {
            // Internal RAM
            return self.ram.get_cpu_memory(addr);
        } else if addr < 0x3FFF {
            // PPU Registers
            addr = (addr % 8) + 0x2000; // Mirrored every 8 bytes
            match addr {
                PPU_CTRL_ADDR => return 0, // Write only
                PPU_MASK_ADDR => return 0, // Write only
                // Status of the PPU 
                PPU_STATUS_ADDR => {
                    if debug {
                        self.ppu_data.status
                    } else {
                        self.ppu_data.scroll_latch = false;
                        self.ppu_data.addr_latch = false;
                        let old_nmi;
                        if self.ppu_data.nmi_occurred {
                            old_nmi = 0x80 | self.ppu_data.status;
                        } else {
                            old_nmi = 0x7F & self.ppu_data.status;
                        }

                        self.ppu_data.nmi_occurred = false;
                        self.ppu_data.status &= 0x7F;

                        old_nmi
                    }
                }
                OAM_ADDR_ADDR => return 0, // Write only
                // Data at specified OAM Address
                OAM_DATA_ADDR => {
                    let tmp = self.ppu_data.oam_addr as usize;
                    self.ppu_data.oam_addr = self.ppu_data.oam_addr.wrapping_add(1);
                    return self.ppu_data.oam[tmp];
                }
                PPU_SCROLL_ADDR => return 0, // Write only
                PPU_ADDR_ADDR => return 0,   // Write only
                // Data at specified PPU Vram Address
                PPU_DATA_ADDR => {
                    if debug {
                        return self.ppu_read(self.ppu_data.vram_addr);
                    }
                    self.ppu_data.data = self.ppu_data.data_buffer;
                    self.ppu_data.data_buffer = self.ppu_read(self.ppu_data.vram_addr);
                    if self.ppu_data.vram_addr >= 0x3F00 {
                        self.ppu_data.data = self.ppu_data.data_buffer;
                    }
                    if self.ppu_data.ctrl & 0x04 != 0 {
                        self.ppu_data.vram_addr += 32;
                    } else {
                        self.ppu_data.vram_addr += 1;
                    }
                    self.ppu_data.data
                }
                _ => return 0, // catch all
            }
        } else {
            // Cartridge space
            if addr < 0x4020 {
                // Read joypad one input one bit at a time
                if addr == JOYPAD_ONE_ADDR {
                    return self.input.read_and_shift_joypad_one();
                } else {
                    return 0;
                }
            } else if addr < 0x8000 {
                // Cart RAM, todo
                return 0;
            } else {
                return self.cart.cpu_read(addr);
            }
        }
    }

    // Read word (2 bytes) little endian
    pub fn read_word_little(&mut self, addr: u16, debug: bool) -> u16 {
        let low: u16 = self.read(addr, debug) as u16;

        let high: u16 = self.read(addr + 1, debug) as u16;

        return (high << 8) + low;
    }

    // Read word (2 bytes) little endian with boundary wrap
    pub fn read_word_little_wrap(&mut self, addr: u16, debug: bool) -> u16 {
        let low: u16 = self.read(addr, debug) as u16;

        let low_addr: u8 = (addr as u8).wrapping_add(1);
        let high: u16 = self.read((addr & 0xFF00) + low_addr as u16, debug) as u16;

        return (high << 8) + low;
    }

    pub fn write(&mut self, mut addr: u16, value: u8) {
        if addr < 0x2000 {
            // Internal RAM
            self.ram.set_cpu_memory(addr, value);
        } else if addr < 0x3FFF {
            // PPU Registers
            addr = (addr % 8) + 0x2000; // Mirrored every 8 bytes
            match addr {
                // PPU Controller 
                PPU_CTRL_ADDR => {
                    self.ppu_data.ctrl = value;
                    self.ppu_data.set_nametable_x_t(value & 0x01);
                    self.ppu_data.set_nametable_y_t((value >> 1) & 0x01);
                }
                // PPU Masking Controller
                PPU_MASK_ADDR => self.ppu_data.mask = value,
                PPU_STATUS_ADDR => return, // Read only
                // Address for oam read / write
                OAM_ADDR_ADDR => self.ppu_data.oam_addr = value,
                // Value to write to oam at address specified
                OAM_DATA_ADDR => self.ppu_data.oam[self.ppu_data.oam_addr as usize] = value,
                // Value of scroll position
                PPU_SCROLL_ADDR => {
                    if !self.ppu_data.scroll_latch {
                        self.ppu_data.fine_x_scroll = value & 0x07;
                        self.ppu_data.set_coarse_x_scroll_t(value >> 3);
                        self.ppu_data.scroll_latch = true;
                    } else {
                        self.ppu_data.set_fine_y_scroll_t(value & 0x07);
                        self.ppu_data.set_coarse_y_scroll_t(value >> 3);
                        self.ppu_data.scroll_latch = false;
                    }
                }
                // Address into PPU Vram
                PPU_ADDR_ADDR => {
                    if !self.ppu_data.addr_latch {
                        self.ppu_data.temp_vram_addr =
                            ((value & 0x3F) as u16) << 8 | (self.ppu_data.temp_vram_addr & 0x00FF);
                        self.ppu_data.addr_latch = true;
                    } else {
                        self.ppu_data.temp_vram_addr =
                            (self.ppu_data.temp_vram_addr & 0xFF00) | (value as u16);
                        self.ppu_data.vram_addr = self.ppu_data.temp_vram_addr;
                        self.ppu_data.addr_latch = false;
                    }
                }
                // Value to write to ppu vram at address specified
                PPU_DATA_ADDR => {
                    self.ppu_write(self.ppu_data.vram_addr, value);
                    if self.ppu_data.ctrl & 0x04 != 0 {
                        self.ppu_data.vram_addr += 32;
                    } else {
                        self.ppu_data.vram_addr += 1;
                    }
                }
                _ => return, // catch all
            }
        } else if addr == OAM_DMA_ADDR {
            // Setup OAM DMA
            self.oam_dma_cpu = true;
            self.oam_dma_ppu = true;
            self.oam_dma_addr = (value as u16) << 8;
        } else if addr == JOYPAD_ONE_ADDR {
            // Latch joypad one read
            if value & 1 != 0 {
                self.input.set_latch(true);
            } else {
                // Do the actual input reading here
                // On actual hardware it happens continually,
                // but there is no point as only A could be read
                self.input.update_input();
                self.input.set_latch(false);
            }
        }
    }

    // Read from PPU Vram
    pub fn ppu_read(&self, addr: u16) -> u8 {
        if addr < 0x2000 {
            return self.cart.ppu_read(addr);
        } else {
            return self.ram.get_ppu_memory(addr);
        }
    }

    // Write to PPU Vram
    pub fn ppu_write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 { // Cannot write chr rom
        } else {
            // let actual_addr = addr - 0x2000;
            self.ram.set_ppu_memory(addr, value);
        }
    }
}
//: }}}
