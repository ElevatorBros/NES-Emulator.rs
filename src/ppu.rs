// Vim folding 
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::bus::*;

//: Ppu {{{
pub struct Ppu {
    pub chr_rom: [u8; 0x2000], // 8KB internal chr rom 
    pub vram: [u8; 0x800], // 2KB internal vram 
    pub pallet: [u8; 0x100], // 256 bytes internal pallet ram 
    pub oam: [u8; 0x100], // 256 bytes internal oam 
    
    pub screen: [u8; 4 * 256 * 240], // screen pixel buffer
    

    pub scanline: i16,
    pub cycle: i16,

    pub render_frame: bool,
}
//: }}}

struct RGBA {
  r: u8,
  g: u8,
  b: u8,
  a: u8
}



//: Ppu Functions {{{
impl Ppu {
    pub fn new() -> Self {
        Self {
            chr_rom: [0; 0x2000],
            vram: [0; 0x800],
            pallet: [0; 0x100],
            oam: [0; 0x100],
            screen: [0x00; 4 * 256 * 240],

            scanline: -1,
            cycle: 0,
            render_frame: false,
        }
    }

    /* CPU Registers */
    // PPU_CTRL
    // 0: disable, 1: enable
    fn get_nmi_enable(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 7)) != 0 }
    // 0: slave, 1: master
    fn get_master_slave(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 6)) != 0 }
    // 0: 8x8, 1: 8x16
    fn get_sprite_size(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 5)) != 0 }
    // 0: $0000; 1: $1000
    fn get_background_tile_select(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 4)) != 0 }
    // 0: $0000; 1: $1000
    fn get_sprite_tile_select(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 3)) != 0 }
    // 0: horizontal; 1: vertical
    fn get_increment_mode(&self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 2)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_base_nametable_addr(&self, bus: &mut Bus) -> u8 { bus.read(PPU_CTRL_ADDR) & 0x04 }

    // PPU_MASK
    // 0: color, 1: greyscale
    fn get_greyscale(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 0)) != 0 }
    // 0: hide; 1: show 
    fn get_background_left_column_enable(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 2)) != 0 }
    // 0: hide, 1: show sprites in leftmost 8 pixels of screen 
    fn get_sprite_left_column_enable(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 1)) != 0 }
    // 0: hide; 1: show 
    fn get_background_enable(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 4)) != 0 }
    // 0: hide, 1: show background in leftmost 8 pixels of screen
    fn get_sprite_enable(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 3)) != 0 }
    // 0: none; 1: emphasize 
    fn get_emphasize_red(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 5)) != 0 }
    // 0: none; 1: emphasize 
    fn get_emphasize_green(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 6)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_emphasize_blue(&self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 7)) != 0 }

    // PPU_STATUS
    // Open bus is weird, make sure to come back to this
    // Only write to the low five bits
    fn set_open_bus(&self, bus: &mut Bus, value:u8) { bus.write(PPU_STATUS_ADDR, (value & 0x1F) | (bus.read(PPU_STATUS_ADDR) & 0xE0)) }
    // Weird as well because of hardware bug, look into sprite evaluation
    fn set_sprite_overflow(&self, bus: &mut Bus, value:bool) { 
        if value {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) | 0x20);
        } else {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) & 0xDF);
        }
    }
    fn set_sprite_hit(&self, bus: &mut Bus, value:bool) { 
        if value {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) | 0x40);
        } else {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) & 0xBF);
        }
    }
    fn set_vblank(&self, bus: &mut Bus, value:bool) { 
        if value {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) | 0x80);
        } else {
            bus.write(PPU_STATUS_ADDR, bus.read(PPU_STATUS_ADDR) & 0x7F);
        }
    }

    // OAM_ADDR
    fn get_oam_addr(&self, bus: &mut Bus) -> u8 { bus.read(OAM_ADDR_ADDR) }

    // OAM_DATA
    fn get_oam_data(&self, bus: &mut Bus) -> u8 { bus.read(OAM_DATA_ADDR) }
    fn set_oam_data(&self, bus: &mut Bus, value: u8) { bus.write(OAM_DATA_ADDR, value) }

    // PPU_SCROLL
    fn get_ppu_scroll(&self, bus: &mut Bus) -> u16 { bus.ppu_current_scroll }

    // PPU_ADDR
    fn get_ppu_addr(&self, bus: &mut Bus) -> u16 { bus.ppu_current_addr }

    // PPU_DATA
    fn get_ppu_data(&self, bus: &mut Bus) -> u8 { bus.read(PPU_DATA_ADDR) }

    // OAM_DMA
    fn copy_to_oam(&mut self, bus: &mut Bus) {
        for i in 0x0..0x100 {
            self.oam[i] = bus.read(bus.oam_dma_addr + (i as u16));
        }
        bus.oam_dma_ppu = false;
    }

    fn render(&mut self) {
        self.render_frame = true;
    }

    fn put_pixel(&mut self, y: u16, x: u16, rgba: RGBA) {
        let offset = 4 * ((y as usize) * 256 + (x as usize));
        if offset > 4 * 256 * 240  - 1 {
            return
        }
        self.screen[(offset + 0) as usize] = rgba.r;
        self.screen[(offset + 1) as usize] = rgba.g;
        self.screen[(offset + 2) as usize] = rgba.b;
        self.screen[(offset + 3) as usize] = rgba.a;
    }

    pub fn clock(&mut self, bus: &mut Bus<'_>) {

        // Check for oam dma
        if bus.oam_dma_ppu {
            self.copy_to_oam(bus);
        }

        if self.scanline == -1 { // pre-render scanline
            
        } else if self.scanline <= 239 { // rendering
            if self.cycle % 8 == 0 {
                self.put_pixel(self.scanline as u16, self.cycle as u16, RGBA{r:0xff,g:0xff,b:0xff,a:0xff});
            }
            if self.cycle == 0 { // idle cycle
                            
            } else if self.cycle <= 256 { // current line tile data fetch

            } else if self.cycle <= 320 { // next line first two tiles
                                     
            } else { // fetch two bytes for unknown reason

            }
        } else if self.scanline == 240 { // post render scanline
                                    
        } else { // vblank

        }

        if self.cycle < 341 {
            self.cycle += 1;
        } else {
            self.cycle = 0;
            if self.scanline < 261 {
                self.scanline += 1;
            } else {
                self.scanline = -1;
                self.render();
            }
        }
    }
}
// }}}
