// Vim folding 
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::Bus;
use crate::output_debug_info;


//: Ppu {{{
pub struct Ppu<'a> {
    pub chr_rom: [u8; 0x2000], // 8KB internal chr rom 
    pub vram: [u8; 0x800], // 2KB internal vram 
    pub pallet: [u8; 0x100], // 256 bytes internal pallet ram 
    pub oam: [u8; 0x100], // 256 bytes internal oam 
    
    pub screen: [u8; 3 * 256 * 240], // screen pixel buffer
}
//: }}}



const PPU_CTRL_ADDR: u16 = 0x2000;
const PPU_MASK_ADDR: u16 = 0x2001;
const PPU_STATUS_ADDR: u16 = 0x2002;
const OAM_ADDR_ADDR: u16 = 0x2003;
const OAM_DATA_ADDR: u16 = 0x2004;
const PPU_SCROLL_ADDR: u16 = 0x2005;
const PPU_ADDR_ADDR: u16 = 0x2006;
const PPU_DATA_ADDR: u16 = 0x2007;

impl Ppu {
    /* CPU Registers */
    // PPU_CTRL
    // 0: disable, 1: enable
    fn get_nmi_enable($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 7)) != 0 }
    // 0: slave, 1: master
    fn get_master_slave($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 6)) != 0 }
    // 0: 8x8, 1: 8x16
    fn get_sprite_size($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 5)) != 0 }
    // 0: $0000; 1: $1000
    fn get_background_tile_select($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 4)) != 0 }
    // 0: $0000; 1: $1000
    fn get_sprite_tile_select($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 3)) != 0 }
    // 0: horizontal; 1: vertical
    fn get_increment_mode($self, bus: &mut Bus) -> bool { (bus.read(PPU_CTRL_ADDR) & (1 << 2)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_base_nametable_addr($self, bus: &mut Bus) -> u8 { (bus.read(PPU_CTRL_ADDR) & 0x04) }

    // PPU_MASK
    // 0: color, 1: greyscale
    fn get_greyscale($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 0)) != 0 }
    // 0: hide; 1: show 
    fn get_background_left_column_enable($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 2)) != 0 }
    // 0: hide, 1: show sprites in leftmost 8 pixels of screen 
    fn get_sprite_left_column_enable($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 1)) != 0 }
    // 0: hide; 1: show 
    fn get_background_enable($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 4)) != 0 }
    // 0: hide, 1: show background in leftmost 8 pixels of screen
    fn get_sprite_enable($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 3)) != 0 }
    // 0: none; 1: emphasize 
    fn get_emphasize_red($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 5)) != 0 }
    // 0: none; 1: emphasize 
    fn get_emphasize_green($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 6)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_emphasize_blue($self, bus: &mut Bus) -> bool { (bus.read(PPU_MASK_ADDR) & (1 << 7)) != 0 }

    // PPU_STATUS
    // Open bus is weird, make sure to come back to this
    // Only write to the low five bits
    fn set_open_bus($self, bus: &mut Bus, value:u8) { bus.write(PPU_STATUS_ADDR, (value & 0x1F) | (bus.read(PPU_STATUS_ADDR) & 0xE0)) }
    // Weird as well because of hardware bug, look into sprite evaluation
    fn set_sprite_overflow($self, bus: &mut Bus, value:bool) { 
        if (value) {
            bus.write(bus.read(PPU_STATUS_ADDR) | 0x20);
        } else {
            bus.write(bus.read(PPU_STATUS_ADDR) & 0xDF);
        }
    }
    fn set_sprite_hit($self, bus: &mut Bus, value:bool) { 
        if (value) {
            bus.write(bus.read(PPU_STATUS_ADDR) | 0x40);
        } else {
            bus.write(bus.read(PPU_STATUS_ADDR) & 0xBF);
        }
    }
    fn set_vblank($self, bus: &mut Bus, value:bool) { 
        if (value) {
            bus.write(bus.read(PPU_STATUS_ADDR) | 0x80);
        } else {
            bus.write(bus.read(PPU_STATUS_ADDR) & 0x7F);
        }
    }

    // OAM_ADDR
    fn get_oam_addr($self, bus: &mut Bus) -> u8 { bus.read(OAM_ADDR_ADDR) }

    // OAM_DATA
    fn get_oam_data($self, bus: &mut Bus) -> u8 { bus.read(OAM_DATA_ADDR) }
    fn set_oam_data($self, bus: &mut Bus, value: u8) { bus.write(OAM_DATA_ADDR, value) }

    // PPU_SCROLL
    fn get_ppu_scroll($self, bus: &mut Bus) -> u8 { bus.read(PPU_SCROLL_ADDR) }

    // PPU_ADDR
    fn get_ppu_addr($self, bus: &mut Bus) -> u8 { bus.read(PPU_ADDR_ADDR) }

    // PPU_DATA
    fn get_ppu_data($self, bus: &mut Bus) -> u8 { bus.read(PPU_DATA_ADDR) }

    // OAM_DMA
    fn copy_to_oam($self, bus: &mut Bus, value: u8) {
       // increase_cpu_clock(); 
       // copy $xx00 - $xxFF to oam where xx = value
    }





    fn clock(&self, bus: &mut Bus) {
    }
}
