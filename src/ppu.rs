// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::bus::*;

//: SpriteData {{{
#[derive(Copy, Clone)]
pub struct SpriteData {
    pub pattern_shift: u8,
    pub latch: u8,
    pub counter: u8,
}
//: }}}

//: Ppu {{{
pub struct Ppu {
    // General internal data
    //pub chr_rom: [u8; 0x2000], // 8KB internal chr rom
    //pub vram: [u8; 0x800],   // 2KB internal vram
    //pub pallet: [u8; 0x100], // 256 bytes internal pallet ram
    //pub data: [u8; 0x4000], // All the data is placed in different parts here

    // Background
    //pub vram_addr: u16,
    //pub vram_addr_tmp: u16,
    // Two tiles, high is the data that is being loaded, low is the data that is being used
    //pub background_pattern_shift: [u16; 2],
    //pub background_pattern_shift_next: [u16; 2],
    // Pallet attributes for low data of the background pattern
    //pub background_palette_shift: [u8; 2],
    //pub background_palette_shift_next: [u8; 2],
    pub background_next_nametable: u8,
    pub background_next_attrib: u8,
    pub background_next_pattern_low: u8,
    pub background_next_pattern_high: u8,

    pub background_shift_attrib_low: u16,
    pub background_shift_attrib_high: u16,

    pub background_shift_pattern_low: u16,
    pub background_shift_pattern_high: u16,

    // Sprites
    pub primary_oam: [u8; 0x100], // 256 bytes internal oam
    pub secondary_oam: [u8; 0x20],
    pub current_sprite_data: [SpriteData; 8],

    // Internal State
    pub scanline: i16,
    pub cycle: i16,
    pub even: bool,

    pub render_frame: bool,
    pub screen: [u8; 4 * 256 * 240], // screen pixel buffer
}
//: }}}

//: RGBA {{{
#[derive(Copy, Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/*
// Load from a .pal file in the future
const PALLET_TO_RGBA: [RGBA; 64] = [
    RGBA {
        r: 0x62,
        g: 0x62,
        b: 0x62,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x1F,
        b: 0xB2,
        a: 0xFF,
    },
    RGBA {
        r: 0x24,
        g: 0x04,
        b: 0xC8,
        a: 0xFF,
    },
    RGBA {
        r: 0x52,
        g: 0x00,
        b: 0xB2,
        a: 0xFF,
    },
    RGBA {
        r: 0x73,
        g: 0x00,
        b: 0x76,
        a: 0xFF,
    },
    RGBA {
        r: 0x80,
        g: 0x00,
        b: 0x24,
        a: 0xFF,
    },
    RGBA {
        r: 0x73,
        g: 0x0B,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x52,
        g: 0x28,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x24,
        g: 0x44,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x57,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x57,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x53,
        b: 0x24,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x3C,
        b: 0x76,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, // Blacker than Black
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0xAB,
        g: 0xAB,
        b: 0xAB,
        a: 0xFF,
    },
    RGBA {
        r: 0x0D,
        g: 0x57,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0x4B,
        g: 0x30,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0x8A,
        g: 0x13,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xBC,
        g: 0x08,
        b: 0xD6,
        a: 0xFF,
    },
    RGBA {
        r: 0xD2,
        g: 0x12,
        b: 0x69,
        a: 0xFF,
    },
    RGBA {
        r: 0xC7,
        g: 0x2E,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x9D,
        g: 0x54,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x60,
        g: 0x7B,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x20,
        g: 0x98,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0xA3,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x99,
        b: 0x42,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x7D,
        b: 0xB4,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0x53,
        g: 0xAE,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0x90,
        g: 0x85,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xD3,
        g: 0x65,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0x57,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0x5D,
        b: 0xCF,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0x77,
        b: 0x57,
        a: 0xFF,
    },
    RGBA {
        r: 0xFA,
        g: 0x9E,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0xBD,
        g: 0xC7,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x7A,
        g: 0xE7,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x43,
        g: 0xF6,
        b: 0x11,
        a: 0xFF,
    },
    RGBA {
        r: 0x26,
        g: 0xEF,
        b: 0x7E,
        a: 0xFF,
    },
    RGBA {
        r: 0x2C,
        g: 0xD5,
        b: 0xF6,
        a: 0xFF,
    },
    RGBA {
        r: 0x4E,
        g: 0x4E,
        b: 0x4E,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xB6,
        g: 0xE1,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xCE,
        g: 0xD1,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xE9,
        g: 0xC3,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xBC,
        b: 0xFF,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xBD,
        b: 0xF4,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xC6,
        b: 0xC3,
        a: 0xFF,
    },
    RGBA {
        r: 0xFF,
        g: 0xD5,
        b: 0x9A,
        a: 0xFF,
    },
    RGBA {
        r: 0xE9,
        g: 0xE6,
        b: 0x81,
        a: 0xFF,
    },
    RGBA {
        r: 0xCE,
        g: 0xF4,
        b: 0x81,
        a: 0xFF,
    },
    RGBA {
        r: 0xB6,
        g: 0xFB,
        b: 0x9A,
        a: 0xFF,
    },
    RGBA {
        r: 0xA9,
        g: 0xFA,
        b: 0xC3,
        a: 0xFF,
    },
    RGBA {
        r: 0xA9,
        g: 0xF0,
        b: 0xF4,
        a: 0xFF,
    },
    RGBA {
        r: 0xB8,
        g: 0xB8,
        b: 0xB8,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    },
];
*/
// }}}

const PALLET_TO_RGBA: [RGBA; 64] = [
    RGBA {
        r: 0x59,
        g: 0x59,
        b: 0x5f,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x8f,
        a: 0xff,
    },
    RGBA {
        r: 0x18,
        g: 0x00,
        b: 0x8f,
        a: 0xff,
    },
    RGBA {
        r: 0x3f,
        g: 0x00,
        b: 0x77,
        a: 0xff,
    },
    RGBA {
        r: 0x50,
        g: 0x00,
        b: 0x50,
        a: 0xff,
    },
    RGBA {
        r: 0x50,
        g: 0x00,
        b: 0x10,
        a: 0xff,
    },
    RGBA {
        r: 0x50,
        g: 0x00,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x40,
        g: 0x20,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x30,
        g: 0x30,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x10,
        g: 0x30,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x30,
        b: 0x10,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x40,
        b: 0x40,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x40,
        b: 0x60,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0xa0,
        g: 0xa0,
        b: 0xa0,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x40,
        b: 0xd0,
        a: 0xff,
    },
    RGBA {
        r: 0x50,
        g: 0x10,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0x70,
        g: 0x00,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0x00,
        b: 0xb0,
        a: 0xff,
    },
    RGBA {
        r: 0xa0,
        g: 0x00,
        b: 0x50,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0x30,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x80,
        g: 0x40,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x60,
        g: 0x60,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x30,
        g: 0x60,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x60,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x60,
        b: 0x50,
        a: 0xff,
    },
    RGBA {
        r: 0x00,
        g: 0x50,
        b: 0x80,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0xe0,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0x40,
        g: 0x80,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0x70,
        g: 0x70,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0x40,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0xb0,
        g: 0x40,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0xc0,
        g: 0x50,
        b: 0x90,
        a: 0xff,
    },
    RGBA {
        r: 0xd0,
        g: 0x60,
        b: 0x40,
        a: 0xff,
    },
    RGBA {
        r: 0xc0,
        g: 0x80,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0xb0,
        g: 0xa0,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x70,
        g: 0xb0,
        b: 0x00,
        a: 0xff,
    },
    RGBA {
        r: 0x20,
        g: 0xb0,
        b: 0x20,
        a: 0xff,
    },
    RGBA {
        r: 0x20,
        g: 0xb0,
        b: 0x70,
        a: 0xff,
    },
    RGBA {
        r: 0x20,
        g: 0xb0,
        b: 0xc0,
        a: 0xff,
    },
    RGBA {
        r: 0x40,
        g: 0x40,
        b: 0x40,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0xe0,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0xc0,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0xa0,
        g: 0xa0,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0xb0,
        g: 0x90,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0xd0,
        g: 0x90,
        b: 0xf0,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0x90,
        b: 0xd0,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0xa0,
        b: 0xa0,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0xb0,
        b: 0x90,
        a: 0xff,
    },
    RGBA {
        r: 0xe0,
        g: 0xd0,
        b: 0x80,
        a: 0xff,
    },
    RGBA {
        r: 0xb0,
        g: 0xd0,
        b: 0x80,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0xd0,
        b: 0x90,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0xd0,
        b: 0xb0,
        a: 0xff,
    },
    RGBA {
        r: 0x90,
        g: 0xd0,
        b: 0xe0,
        a: 0xff,
    },
    RGBA {
        r: 0xa0,
        g: 0xa0,
        b: 0xa0,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
    RGBA {
        r: 0x08,
        g: 0x08,
        b: 0x08,
        a: 0xff,
    },
];

//: Ppu Functions {{{
impl Ppu {
    pub fn new() -> Self {
        Self {
            //chr_rom: [0; 0x2000],
            //vram: [0; 0x800],
            //pallet: [0; 0x100],
            //data: [0; 0x4000],
            //vram_addr: 0,
            //vram_addr_tmp: 0,

            // background_pattern_shift: [0; 2],
            // background_pattern_shift_next: [0; 2],
            // background_palette_shift: [0; 2],
            // background_palette_shift_next: [0; 2],
            background_next_nametable: 0,
            background_next_attrib: 0,
            background_next_pattern_low: 0,
            background_next_pattern_high: 0,

            background_shift_attrib_low: 0,
            background_shift_attrib_high: 0,

            background_shift_pattern_low: 0,
            background_shift_pattern_high: 0,

            primary_oam: [0; 0x100], // 256 bytes internal oam
            secondary_oam: [0; 0x20],
            current_sprite_data: [SpriteData {
                pattern_shift: 0,
                latch: 0,
                counter: 0,
            }; 8],

            scanline: -1,
            cycle: 0,
            even: true,

            render_frame: false,
            screen: [0x00; 4 * 256 * 240],
        }
    }

    /* CPU Registers */
    // PPU_CTRL
    // 0: disable, 1: enable
    fn get_nmi_enable(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 7)) != 0
    }
    // 0: slave, 1: master
    fn get_master_slave(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 6)) != 0
    }
    // 0: 8x8, 1: 8x16
    fn get_sprite_size(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 5)) != 0
    }
    // 0: $0000; 1: $1000
    fn get_background_table_select(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 4)) != 0
    }
    // 0: $0000; 1: $1000
    fn get_sprite_table_select(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 3)) != 0
    }
    // 0: horizontal; 1: vertical
    fn get_increment_mode(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 2)) != 0
    }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_base_nametable_addr(&self, bus: &mut Bus) -> u8 {
        bus.ppu_data.ctrl & 0x04
    }

    // PPU_MASK
    // 0: color, 1: greyscale
    fn get_greyscale(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 0)) != 0
    }
    // 0: hide; 1: show
    fn get_background_left_column_enable(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 2)) != 0
    }
    // 0: hide, 1: show sprites in leftmost 8 pixels of screen
    fn get_sprite_left_column_enable(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 1)) != 0
    }
    // 0: hide; 1: show
    fn get_background_enable(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 4)) != 0
    }
    // 0: hide, 1: show background in leftmost 8 pixels of screen
    fn get_sprite_enable(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 3)) != 0
    }
    // 0: none; 1: emphasize
    fn get_emphasize_red(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 5)) != 0
    }
    // 0: none; 1: emphasize
    fn get_emphasize_green(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 6)) != 0
    }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_emphasize_blue(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.mask & (1 << 7)) != 0
    }

    // PPU_STATUS
    // Open bus is weird, make sure to come back to this
    // Only write to the low five bits
    fn set_open_bus(&self, bus: &mut Bus, value: u8) {
        bus.ppu_data.status = (value & 0x1F) | (bus.ppu_data.status & 0xE0)
    }
    // Weird as well because of hardware bug, look into sprite evaluation
    fn set_sprite_overflow(&self, bus: &mut Bus, value: bool) {
        if value {
            bus.ppu_data.status = bus.ppu_data.status | 0x20;
        } else {
            bus.ppu_data.status = bus.ppu_data.status & 0xDF;
        }
    }
    fn set_sprite_hit(&self, bus: &mut Bus, value: bool) {
        if value {
            bus.ppu_data.status = bus.ppu_data.status | 0x40;
        } else {
            bus.ppu_data.status = bus.ppu_data.status & 0xBF;
        }
    }
    fn set_vblank(&self, bus: &mut Bus, value: bool) {
        if value {
            bus.ppu_data.status = bus.ppu_data.status | 0x80;
        } else {
            bus.ppu_data.status = bus.ppu_data.status & 0x7F;
        }
    }

    // OAM_ADDR
    fn get_oam_addr(&self, bus: &mut Bus) -> u8 {
        bus.ppu_data.oam_addr
    }

    // OAM_DATA
    fn get_oam_data(&self, bus: &mut Bus) -> u8 {
        bus.ppu_data.oam_data
    }
    fn set_oam_data(&self, bus: &mut Bus, value: u8) {
        bus.ppu_data.oam_data = value
    }

    // PPU_SCROLL
    /*fn get_ppu_scroll(&self, bus: &mut Bus) -> u16 {
        bus.ppu_data.scroll
    }*/

    // PPU_ADDR
    /*fn get_ppu_addr(&self, bus: &mut Bus) -> u16 {
        bus.ppu_data.addr
    }*/

    // PPU_DATA
    fn get_ppu_data(&self, bus: &mut Bus) -> u8 {
        bus.ppu_data.data
    }

    // OAM_DMA
    fn copy_to_oam(&mut self, bus: &mut Bus) {
        for i in 0x0..0x100 {
            self.primary_oam[i] = bus.read(bus.oam_dma_addr + (i as u16), false);
        }
        bus.oam_dma_ppu = false;
    }

    fn get_rgba(&mut self, pixel: u8, pallet: u8, bus: &mut Bus) -> RGBA {
        let addr = 0x3F00 + ((pallet as u16) << 2) + (pixel as u16);
        PALLET_TO_RGBA[(bus.ppu_read(addr) & 0x3F) as usize]
    }

    fn render(&mut self) {
        self.render_frame = true;
    }

    fn put_pixel(&mut self, y: u16, x: u16, rgba: RGBA) {
        let offset = 4 * ((y as usize) * 256 + (x as usize));
        if offset > 4 * 256 * 240 - 1 {
            return;
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

        if self.scanline == 0
            && self.cycle == 0
            && !self.even
            && (self.get_sprite_enable(bus) || self.get_background_enable(bus))
        {
            // Skip a clock cycle on cycle 0 scanline 0 if we are on an even frame and rendering
            self.cycle += 1;
        }

        if self.scanline <= 239 || self.scanline == 261 {
            // Shift
            if self.get_background_enable(bus) {
                self.background_shift_pattern_low <<= 1;
                self.background_shift_pattern_high <<= 1;

                self.background_shift_attrib_low <<= 1;
                self.background_shift_attrib_high <<= 1;
            }

            if self.scanline == 261 {
                // pre-render scanline
                if self.cycle == 1 {
                    bus.ppu_data.nmi_occurred = false;
                    self.set_vblank(bus, false);
                    self.set_sprite_hit(bus, false);
                    self.set_sprite_overflow(bus, false);

                    if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                        PpuData::set_fine_y_scroll(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_fine_y_scroll(bus.ppu_data.temp_vram_addr),
                        );
                        PpuData::set_nametable_y(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_nametable_y(bus.ppu_data.temp_vram_addr),
                        );
                        PpuData::set_coarse_y_scroll(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_coarse_y_scroll(bus.ppu_data.temp_vram_addr),
                        );
                    }
                }
            }
            // rendering
            if self.cycle == 0 { // idle cycle
            } else if self.cycle <= 256 {
                // current line tile data fetch
                match self.cycle % 8 {
                    0 => {
                        if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                            if PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) == 31 {
                                PpuData::set_coarse_x_scroll(&mut bus.ppu_data.vram_addr, 0);
                                if PpuData::get_nametable_x(bus.ppu_data.vram_addr) == 0 {
                                    PpuData::set_nametable_x(&mut bus.ppu_data.vram_addr, 0);
                                } else {
                                    PpuData::set_nametable_x(&mut bus.ppu_data.vram_addr, 1);
                                }
                            } else {
                                let tmp = PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) + 1;
                                PpuData::set_coarse_x_scroll(&mut bus.ppu_data.vram_addr, tmp);
                            }
                        }
                    }
                    1 => {
                        //Load shift
                        self.background_shift_pattern_low = (self.background_shift_pattern_low
                            & 0xFF00)
                            | (self.background_next_pattern_low as u16);
                        self.background_shift_pattern_high = (self.background_shift_pattern_high
                            & 0xFF00)
                            | (self.background_next_pattern_high as u16);

                        if self.background_next_attrib & 0x01 != 0 {
                            self.background_shift_attrib_low |= 0x00FF;
                        } else {
                            self.background_shift_attrib_low &= 0xFF00;
                        }
                        if self.background_next_attrib & 0x02 != 0 {
                            self.background_shift_attrib_high |= 0x00FF;
                        } else {
                            self.background_shift_attrib_high &= 0xFF00;
                        }
                        // Nametable
                        self.background_next_nametable =
                            bus.ppu_read(0x2000 + (bus.ppu_data.vram_addr & 0x0FFF));
                    }
                    3 => {
                        // Attribute
                        let mut attrib_addr: u16 = 0x23C0;
                        attrib_addr |=
                            (PpuData::get_nametable_y(bus.ppu_data.vram_addr) as u16) << 11;
                        attrib_addr |=
                            (PpuData::get_nametable_x(bus.ppu_data.vram_addr) as u16) << 10;
                        attrib_addr |=
                            ((PpuData::get_coarse_y_scroll(bus.ppu_data.vram_addr) as u16) >> 2)
                                << 3;
                        attrib_addr |=
                            (PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) as u16) >> 2;
                        self.background_next_attrib = bus.ppu_read(attrib_addr);

                        if PpuData::get_coarse_y_scroll(bus.ppu_data.vram_addr) & 0x02 != 0 {
                            self.background_next_attrib >>= 4;
                        }
                        if PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) & 0x02 != 0 {
                            self.background_next_attrib >>= 2;
                        }
                        self.background_next_attrib &= 0x03;
                    }
                    5 => {
                        // Pattern low
                        let mut pattern_addr: u16 =
                            PpuData::get_fine_y_scroll(bus.ppu_data.vram_addr) as u16;

                        pattern_addr += (self.background_next_nametable as u16) << 4;

                        if self.get_background_table_select(bus) {
                            pattern_addr += 0x1000;
                        }

                        self.background_next_pattern_low = bus.ppu_read(pattern_addr);
                    }
                    7 => {
                        // Pattern high
                        let mut pattern_addr: u16 =
                            PpuData::get_fine_y_scroll(bus.ppu_data.vram_addr) as u16;

                        pattern_addr += (self.background_next_nametable as u16) << 4;

                        if self.get_background_table_select(bus) {
                            pattern_addr += 0x1000;
                        }

                        pattern_addr += 8;

                        self.background_next_pattern_high = bus.ppu_read(pattern_addr);
                    }
                    _ => {}
                }

                if self.cycle == 256 {
                    if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                        if PpuData::get_fine_y_scroll(bus.ppu_data.vram_addr) < 7 {
                            let tmp = PpuData::get_fine_y_scroll(bus.ppu_data.vram_addr) + 1;
                            PpuData::set_fine_y_scroll(&mut bus.ppu_data.vram_addr, tmp);
                        } else {
                            PpuData::set_fine_y_scroll(&mut bus.ppu_data.vram_addr, 0);

                            if PpuData::get_coarse_y_scroll(bus.ppu_data.vram_addr) == 29 {
                                PpuData::set_coarse_y_scroll(&mut bus.ppu_data.vram_addr, 0);
                                if PpuData::get_nametable_y(bus.ppu_data.vram_addr) == 0 {
                                    PpuData::set_nametable_y(&mut bus.ppu_data.vram_addr, 0);
                                } else {
                                    PpuData::set_nametable_y(&mut bus.ppu_data.vram_addr, 1);
                                }
                            } else if PpuData::get_coarse_y_scroll(bus.ppu_data.vram_addr) == 31 {
                                PpuData::set_coarse_y_scroll(&mut bus.ppu_data.vram_addr, 0);
                            } else {
                                let tmp = PpuData::get_coarse_y_scroll(bus.ppu_data.vram_addr) + 1;
                                PpuData::set_coarse_y_scroll(&mut bus.ppu_data.vram_addr, tmp);
                            }
                        }
                    }
                }
            } else if self.cycle <= 320 {
                if self.cycle == 257 {
                    //Load shift
                    self.background_shift_pattern_low = (self.background_shift_pattern_low
                        & 0xFF00)
                        | (self.background_next_pattern_low as u16);
                    self.background_shift_pattern_high = (self.background_shift_pattern_high
                        & 0xFF00)
                        | (self.background_next_pattern_high as u16);

                    if self.background_next_attrib & 0x01 != 0 {
                        self.background_shift_attrib_low |= 0x00FF;
                    } else {
                        self.background_shift_attrib_low &= 0xFF00;
                    }
                    if self.background_next_attrib & 0x02 != 0 {
                        self.background_shift_attrib_high |= 0x00FF;
                    } else {
                        self.background_shift_attrib_high &= 0xFF00;
                    }

                    if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                        PpuData::set_nametable_x(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_nametable_x(bus.ppu_data.temp_vram_addr),
                        );
                        PpuData::set_coarse_x_scroll(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_coarse_x_scroll(bus.ppu_data.temp_vram_addr),
                        );
                    }
                }

                // Technicaly this happens multiple times but we should only need to do it once.
                if self.scanline == 261 && self.cycle == 280 {
                    // I don't think we need to do the fine_y_scroll, but if stuff dosn't work try
                    // that
                    if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                        PpuData::set_nametable_y(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_nametable_y(bus.ppu_data.temp_vram_addr),
                        );
                        PpuData::set_coarse_y_scroll(
                            &mut bus.ppu_data.vram_addr,
                            PpuData::get_coarse_y_scroll(bus.ppu_data.temp_vram_addr),
                        );
                    }
                }
            } else if self.cycle <= 336 {
                // next line first two tiles
                if self.cycle == 328 || self.cycle == 336 {
                    if self.get_background_enable(bus) || self.get_sprite_enable(bus) {
                        if PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) == 31 {
                            PpuData::set_coarse_x_scroll(&mut bus.ppu_data.vram_addr, 0);
                            if PpuData::get_nametable_x(bus.ppu_data.vram_addr) == 0 {
                                PpuData::set_nametable_x(&mut bus.ppu_data.vram_addr, 0);
                            } else {
                                PpuData::set_nametable_x(&mut bus.ppu_data.vram_addr, 1);
                            }
                        } else {
                            let tmp = PpuData::get_coarse_x_scroll(bus.ppu_data.vram_addr) + 1;
                            PpuData::set_coarse_x_scroll(&mut bus.ppu_data.vram_addr, tmp);
                        }
                    }
                }
            } else { // fetch two bytes for unknown reason
            }

            let mut background_pixel = 0x00;
            let mut background_palette = 0x00;

            // Doing the pixel stuff
            if self.get_background_enable(bus) {
                let mux = 0x8000 >> bus.ppu_data.fine_x_scroll;

                if self.background_shift_pattern_low & mux != 0 {
                    background_pixel |= 0x01;
                }
                if self.background_shift_pattern_high & mux != 0 {
                    background_pixel |= 0x02;
                }

                if self.background_shift_attrib_low & mux != 0 {
                    background_palette |= 0x01;
                }
                if self.background_shift_attrib_high & mux != 0 {
                    background_palette |= 0x02;
                }
            }
            let true_pixel = self.get_rgba(background_pixel, background_palette, bus);
            //let true_pixel = PALLET_TO_RGBA[(self.background_next_nametable % 64) as usize];
            self.put_pixel(self.scanline as u16, self.cycle as u16, true_pixel);
        } else if self.scanline == 240 { // post render scanline
        } else if self.scanline <= 260 {
            // vblank
            if self.scanline == 241 && self.cycle == 1 {
                // Set nmi
                self.set_vblank(bus, true);
                bus.ppu_data.nmi_occurred = true;
                if self.get_nmi_enable(bus) {
                    bus.nmi_signal = true;
                }
            }
        } else {
            // error
        }

        if self.cycle < 341 {
            self.cycle += 1;
        } else {
            self.cycle = 0;
            if self.scanline < 262 {
                self.scanline += 1;
            } else {
                self.scanline = 0;
                self.even = !self.even;
                self.render();
            }
        }
    }
}
// }}}
