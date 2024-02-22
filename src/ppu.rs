// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::bus::*;
use std::cell::RefCell;
use std::rc::Rc;

//: Overview Comment {{{ 
/* The PPU is a complex mess, so let me try and give an overview here.
 * Every PPU clock cycle (not during blanking) outputs a single pixel to the screen.
 * There are two ways to put stuff on the screen, background and sprites.
 *
 * Background is a series of index in a nametable which reference tiles stored on the cart (pattern
 * tables). The ppu fetches the right tile from the nametable by looking at a specific spot (based on
 * scrolling) pulling that tile, getting a pixel from that tile, then getting the palette and any
 * special attributes, putting that all together and getting a pixel. 
 *
 * Sprites are tiles that can be placed at any spot. Up to 8 per line and 64 total, stored in the
 * OAM. Sprite pixel fetches happen similarly to background tile fetches.
 *
 * Background and Sprites have priority to determine which pixel goes to the screen.
 * Each tile has 4 pixel indices (00, 01, 10, 11) and those will index into a pallet of four pallet 
 * indexes, which index into the NES's internal possible colors. Index 00 is sort of "transparent" 
 * (background will show through 00 parts of a sprite tile for example).
 *
 * The ppu is constantly reading and written, so we have shift register's which store data for the 
 * next thing, and they get swapped in and then shifted down.
 *
 * There is an H blank at the end of each line for the ppu to read sprite data for the next line,
 * and a V blank at the end of the last line so the CPU has time to fill data for the next frame.
 * */
//: }}}

//: Ppu {{{
pub struct Ppu<'a> {
    // Data for next tile
    // Nametable index
    pub background_next_nametable: u8,
    // Tile attributes
    pub background_next_attrib: u8,
    // Pattern low bit
    pub background_next_pattern_low: u8,
    // Pattern high bit
    pub background_next_pattern_high: u8,

    // Current attribute and pattern data being shifted.
    // There is no nametable shift because it is rolled into pattern shift
    pub background_shift_attrib_low: u16,
    pub background_shift_attrib_high: u16,
    pub background_shift_pattern_low: u16,
    pub background_shift_pattern_high: u16,

    // Sprites
    // Used for sprite search
    pub num_next_sprites_found: u8,
    // Current sprite we are testing
    pub search_oam_index: u16,
    // Which of the 4 bytes in the oam index we are currently at
    pub search_current_sprite_byte: i8,
    // Sprite zero is on the current scanline flag
    pub sprite_zero_on_scanline: bool,

    // Sprites for the next scanline
    pub next_scanline_sprites: [[u8; 4]; 9], // 4 bytes for each of the 8 sprites (+1 overflow)
    // Sprites on the current scanline
    pub current_scanline_sprites: [[u8; 4]; 8],

    // Current sprite pattern shift (these actually get muxed)
    pub scanline_sprite_patterns_low: [u8; 8],
    pub scanline_sprite_patterns_high: [u8; 8],
    // Sprite counters, used to tell when to start rendering sprite
    pub scanline_sprite_shift_counters: [u8; 8],

    // Internal State
    // scanline is the current line on the screen
    pub scanline: i16,
    // cycle is the current column on the screen
    pub cycle: i16,
    // Even or odd frame flag
    pub even: bool,

    // Flag when frame is ready to render
    pub render_frame: bool,
    // RGBA representation of the screen
    pub screen: [u8; 4 * 256 * 240], // screen pixel buffer

    // Debug Stuff, Representation of left and right pattern tables
    pub pattern_table_left: [u8; 4 * 128 * 128],
    pub pattern_table_right: [u8; 4 * 128 * 128],

    // Reference to main bus 
    pub bus: Rc<RefCell<Bus<'a>>>, 
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

// Pallet index to RGBA value lookup
const PALLET_TO_RGBA: [RGBA; 64] = [
    RGBA{r:0x59,g:0x59,b:0x5f,a:0xff,},RGBA{r:0x00,g:0x00,b:0x8f,a:0xff,},RGBA{r:0x18,g:0x00,b:0x8f,a:0xff,},RGBA{r:0x3f,g:0x00,b:0x77,a:0xff,},
    RGBA{r:0x50,g:0x00,b:0x50,a:0xff,},RGBA{r:0x50,g:0x00,b:0x10,a:0xff,},RGBA{r:0x50,g:0x00,b:0x00,a:0xff,},RGBA{r:0x40,g:0x20,b:0x00,a:0xff,},
    RGBA{r:0x30,g:0x30,b:0x00,a:0xff,},RGBA{r:0x10,g:0x30,b:0x00,a:0xff,},RGBA{r:0x00,g:0x30,b:0x10,a:0xff,},RGBA{r:0x00,g:0x40,b:0x40,a:0xff,},
    RGBA{r:0x00,g:0x40,b:0x60,a:0xff,},RGBA{r:0x00,g:0x00,b:0x00,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},
    RGBA{r:0xa0,g:0xa0,b:0xa0,a:0xff,},RGBA{r:0x00,g:0x40,b:0xd0,a:0xff,},RGBA{r:0x50,g:0x10,b:0xe0,a:0xff,},RGBA{r:0x70,g:0x00,b:0xe0,a:0xff,},
    RGBA{r:0x90,g:0x00,b:0xb0,a:0xff,},RGBA{r:0xa0,g:0x00,b:0x50,a:0xff,},RGBA{r:0x90,g:0x30,b:0x00,a:0xff,},RGBA{r:0x80,g:0x40,b:0x00,a:0xff,},
    RGBA{r:0x60,g:0x60,b:0x00,a:0xff,},RGBA{r:0x30,g:0x60,b:0x00,a:0xff,},RGBA{r:0x00,g:0x60,b:0x00,a:0xff,},RGBA{r:0x00,g:0x60,b:0x50,a:0xff,},
    RGBA{r:0x00,g:0x50,b:0x80,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},
    RGBA{r:0xe0,g:0xe0,b:0xe0,a:0xff,},RGBA{r:0x40,g:0x80,b:0xf0,a:0xff,},RGBA{r:0x70,g:0x70,b:0xf0,a:0xff,},RGBA{r:0x90,g:0x40,b:0xf0,a:0xff,},
    RGBA{r:0xb0,g:0x40,b:0xe0,a:0xff,},RGBA{r:0xc0,g:0x50,b:0x90,a:0xff,},RGBA{r:0xd0,g:0x60,b:0x40,a:0xff,},RGBA{r:0xc0,g:0x80,b:0x00,a:0xff,},
    RGBA{r:0xb0,g:0xa0,b:0x00,a:0xff,},RGBA{r:0x70,g:0xb0,b:0x00,a:0xff,},RGBA{r:0x20,g:0xb0,b:0x20,a:0xff,},RGBA{r:0x20,g:0xb0,b:0x70,a:0xff,},
    RGBA{r:0x20,g:0xb0,b:0xc0,a:0xff,},RGBA{r:0x40,g:0x40,b:0x40,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},
    RGBA{r:0xe0,g:0xe0,b:0xe0,a:0xff,},RGBA{r:0x90,g:0xc0,b:0xf0,a:0xff,},RGBA{r:0xa0,g:0xa0,b:0xf0,a:0xff,},RGBA{r:0xb0,g:0x90,b:0xf0,a:0xff,},
    RGBA{r:0xd0,g:0x90,b:0xf0,a:0xff,},RGBA{r:0xe0,g:0x90,b:0xd0,a:0xff,},RGBA{r:0xe0,g:0xa0,b:0xa0,a:0xff,},RGBA{r:0xe0,g:0xb0,b:0x90,a:0xff,},
    RGBA{r:0xe0,g:0xd0,b:0x80,a:0xff,},RGBA{r:0xb0,g:0xd0,b:0x80,a:0xff,},RGBA{r:0x90,g:0xd0,b:0x90,a:0xff,},RGBA{r:0x90,g:0xd0,b:0xb0,a:0xff,},
    RGBA{r:0x90,g:0xd0,b:0xe0,a:0xff,},RGBA{r:0xa0,g:0xa0,b:0xa0,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},RGBA{r:0x08,g:0x08,b:0x08,a:0xff,},
];
// }}}

//: PpuData {{{
// PPU data needed by the bus
pub struct PpuData {
    // NMI flag from ppu (vblank stuff)
    pub nmi_occurred: bool,

    // PPU registers
    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub scroll_latch: bool,
    pub addr_latch: bool,
    pub data: u8,
    pub data_buffer: u8,

    // Scroll 
    pub fine_x_scroll: u8,
    // Vram current address
    pub vram_addr: u16,
    pub temp_vram_addr: u16,

    // OAM
    pub oam: [u8; 0x100], // 256 bytes internal oam
}
// }}}

//: PpuData Functions {{{
impl PpuData {
    // Vram addr functions
    pub fn get_fine_y_scroll_v(&self) -> u8 { ((self.vram_addr >> 12) & 0x0007) as u8 }
    pub fn get_fine_y_scroll_t(&self) -> u8 { ((self.temp_vram_addr >> 12) & 0x0007) as u8 }
    pub fn get_nametable_y_v(&self) -> u8 { ((self.vram_addr >> 11) & 0x0001) as u8 }
    pub fn get_nametable_y_t(&self) -> u8 { ((self.temp_vram_addr >> 11) & 0x0001) as u8 }
    pub fn get_nametable_x_v(&self) -> u8 { ((self.vram_addr >> 10) & 0x0001) as u8 }
    pub fn get_nametable_x_t(&self) -> u8 { ((self.temp_vram_addr >> 10) & 0x0001) as u8 }
    pub fn get_coarse_y_scroll_v(&self) -> u8 { ((self.vram_addr >> 5) & 0x001F) as u8 }
    pub fn get_coarse_y_scroll_t(&self) -> u8 { ((self.temp_vram_addr >> 5) & 0x001F) as u8 }
    pub fn get_coarse_x_scroll_v(&self) -> u8 { (self.vram_addr & 0x001F) as u8 }
    pub fn get_coarse_x_scroll_t(&self) -> u8 { (self.temp_vram_addr & 0x001F) as u8 }

    pub fn set_fine_y_scroll_v(&mut self, value: u8) { self.vram_addr = (self.vram_addr & 0x0FFF) | ((value as u16) << 12); }
    pub fn set_fine_y_scroll_t(&mut self, value: u8) { self.temp_vram_addr = (self.temp_vram_addr & 0x0FFF) | ((value as u16) << 12); }
    pub fn set_nametable_y_v(&mut self, value: u8) { self.vram_addr = (self.vram_addr & 0x77FF) | ((value as u16) << 11); }
    pub fn set_nametable_y_t(&mut self, value: u8) { self.temp_vram_addr = (self.temp_vram_addr & 0x77FF) | ((value as u16) << 11); }
    pub fn set_nametable_x_v(&mut self, value: u8) { self.vram_addr = (self.vram_addr & 0x7BFF) | ((value as u16) << 10); }
    pub fn set_nametable_x_t(&mut self, value: u8) { self.temp_vram_addr = (self.temp_vram_addr & 0x7BFF) | ((value as u16) << 10); }
    pub fn set_coarse_y_scroll_v(&mut self, value: u8) { self.vram_addr = (self.vram_addr & 0x7C1F) | ((value as u16) << 5); }
    pub fn set_coarse_y_scroll_t(&mut self, value: u8) { self.temp_vram_addr = (self.temp_vram_addr & 0x7C1F) | ((value as u16) << 5); }
    pub fn set_coarse_x_scroll_v(&mut self, value: u8) { self.vram_addr = (self.vram_addr & 0x7FE0) | (value as u16); }
    pub fn set_coarse_x_scroll_t(&mut self, value: u8) { self.temp_vram_addr = (self.temp_vram_addr & 0x7FE0) | (value as u16); }

    // Register functions
    /* CPU Registers */
    // PPU_CTRL
    // 0: disable, 1: enable
    fn get_nmi_enable(&self) -> bool { (self.ctrl & (1 << 7)) != 0 }
    // 0: slave, 1: master
    fn get_master_slave(&self) -> bool { (self.ctrl & (1 << 6)) != 0 }
    // 0: 8x8, 1: 8x16
    fn get_sprite_size(&self) -> bool { (self.ctrl & (1 << 5)) != 0 }
    // 0: $0000; 1: $1000
    fn get_background_table_select(&self) -> bool { (self.ctrl & (1 << 4)) != 0 }
    // 0: $0000; 1: $1000
    fn get_sprite_table_select(&self) -> bool { (self.ctrl & (1 << 3)) != 0 }
    // 0: horizontal; 1: vertical
    fn get_increment_mode(&self) -> bool { (self.ctrl & (1 << 2)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_base_nametable_addr(&self) -> u8 { self.ctrl & 0x04 }

    // PPU_MASK
    // 0: color, 1: greyscale
    fn get_greyscale(&self) -> bool { (self.mask & (1 << 0)) != 0 }
    // 0: hide; 1: show
    fn get_background_left_column_enable(&self) -> bool { (self.mask & (1 << 1)) != 0 }
    // 0: hide, 1: show sprites in leftmost 8 pixels of screen
    fn get_sprite_left_column_enable(&self) -> bool { (self.mask & (1 << 2)) != 0 }
    // 0: hide; 1: show
    fn get_background_enable(&self) -> bool { (self.mask & (1 << 3)) != 0 }
    // 0: hide, 1: show background in leftmost 8 pixels of screen
    fn get_sprite_enable(&self) -> bool { (self.mask & (1 << 4)) != 0 }
    // 0: none; 1: emphasize
    fn get_emphasize_red(&self) -> bool { (self.mask & (1 << 5)) != 0 }
    // 0: none; 1: emphasize
    fn get_emphasize_green(&self) -> bool { (self.mask & (1 << 6)) != 0 }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_emphasize_blue(&self) -> bool { (self.mask & (1 << 7)) != 0 }

    // PPU_STATUS
    // Open bus is weird, TODO: make sure to come back to this
    // Only write to the low five bits
    fn set_open_bus(&mut self, value: u8) { self.status = (value & 0x1F) | (self.status & 0xE0) }
    // Weird as well because of hardware bug, look into sprite evaluation
    fn set_sprite_overflow(&mut self, value: bool) { if value { self.status = self.status | 0x20; } else { self.status = self.status & 0xDF; } }
    fn set_sprite_hit(&mut self, value: bool) { if value { self.status = self.status | 0x40; } else { self.status = self.status & 0xBF; } }
    fn set_vblank(&mut self, value: bool) { if value { self.status = self.status | 0x80; } else { self.status = self.status & 0x7F; } }

    // OAM_ADDR
    fn get_oam_addr(&self) -> u8 { self.oam_addr }

    // OAM_DATA
    fn get_oam_data(&self) -> u8 { self.oam[self.oam_addr as usize] }
    fn set_oam_data(&mut self, value: u8) { self.oam[self.oam_addr as usize] = value }

    // PPU_DATA
    fn get_ppu_data(&self) -> u8 { self.data }
}
// }}}

//: Ppu Functions {{{
impl<'a> Ppu<'a> {
    pub fn new(bus: Rc<RefCell<Bus<'a>>>) -> Self {
        Self {
            background_next_nametable: 0,
            background_next_attrib: 0,
            background_next_pattern_low: 0,
            background_next_pattern_high: 0,

            background_shift_attrib_low: 0,
            background_shift_attrib_high: 0,

            background_shift_pattern_low: 0,
            background_shift_pattern_high: 0,

            num_next_sprites_found: 0,
            search_oam_index: 0,
            search_current_sprite_byte: -1,

            sprite_zero_on_scanline: false,

            current_scanline_sprites: [[0; 4]; 8], 
            next_scanline_sprites: [[0; 4]; 9],   
            scanline_sprite_patterns_low: [0; 8],
            scanline_sprite_patterns_high: [0; 8],
            scanline_sprite_shift_counters: [0; 8],

            scanline: 0,
            cycle: 0,
            even: true,

            render_frame: false,
            screen: [0x00; 4 * 256 * 240],

            pattern_table_left: [0x00; 4 * 128 * 128],
            pattern_table_right: [0x00; 4 * 128 * 128],

            bus,
        }
    }

    // OAM indexing functions
    fn get_oam_sprite_y(&self, index: u8) -> u8 { self.bus.borrow().ppu_data.oam[(index * 16 + 0) as usize] }
    fn get_oam_sprite_tile(&self, index: u8) -> u8 { self.bus.borrow().ppu_data.oam[(index * 16 + 1) as usize] }
    fn get_oam_sprite_attr(&self, index: u8) -> u8 { self.bus.borrow().ppu_data.oam[(index * 16 + 2) as usize] }
    fn get_oam_sprite_x(&self, index: u8) -> u8 { self.bus.borrow().ppu_data.oam[(index * 16 + 3) as usize] }

    // Debug functions to convert the pattern tables to a RGBA array that can by printed to screen
    pub fn fill_pattern_tables(&mut self) {
        let bus = self.bus.borrow();
        // Loop through each pixel on each tile for each plane
        for tile_y in 0..16 {
            for tile_x in 0..16 {
                for pixel_y in 0..8 {
                    for pixel_x in 0..8 {
                        for side in 0..2 {
                            let mut bit_plane_addr = 0;
                            if side == 1 {
                                bit_plane_addr = 0x1000;
                            }
                            bit_plane_addr += (tile_y * 256) + (tile_x * 16) + pixel_y;

                            let pixel_low = bus.ppu_read(bit_plane_addr) & (1 << pixel_x);
                            let pixel_high = bus.ppu_read(bit_plane_addr + 8) & (1 << pixel_x);

                            let mut pixel = 0x00;

                            if pixel_low != 0 {
                                pixel |= 0x01;
                            }

                            if pixel_high != 0 {
                                pixel |= 0x02;
                            }

                            let pixel_loc =
                                (((tile_y * 1024) + (tile_x * 8) + (pixel_y * 128) + (7 - pixel_x))
                                    * 4) as usize;

                            // Default to first palette
                            let palette = 0;

                            let true_pixel = PALLET_TO_RGBA[(bus
                                .ppu_read(0x3F00 + ((palette as u16) << 2) + (pixel as u16))
                                & 0x3F)
                                as usize];

                            if side == 0 {
                                self.pattern_table_left[pixel_loc + 0] = true_pixel.r;
                                self.pattern_table_left[pixel_loc + 1] = true_pixel.g;
                                self.pattern_table_left[pixel_loc + 2] = true_pixel.b;
                                self.pattern_table_left[pixel_loc + 3] = true_pixel.a;
                            } else {
                                self.pattern_table_right[pixel_loc + 0] = true_pixel.r;
                                self.pattern_table_right[pixel_loc + 1] = true_pixel.g;
                                self.pattern_table_right[pixel_loc + 2] = true_pixel.b;
                                self.pattern_table_right[pixel_loc + 3] = true_pixel.a;
                            }
                        }
                    }
                }
            }
        }
    }

    // Initialization settings reset each frame
    pub fn pre_render_setup(&mut self) {
        let mut bus = self.bus.borrow_mut();
        // No longer in vblank
        bus.ppu_data.nmi_occurred = false;
        bus.ppu_data.set_vblank(false);
        bus.ppu_data.set_sprite_hit(false);
        bus.ppu_data.set_sprite_overflow(false);

        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            // Copy temporary vram addr to main vram addr
            let t = bus.ppu_data.get_fine_y_scroll_t();
            bus.ppu_data.set_fine_y_scroll_v(t);
            let t = bus.ppu_data.get_nametable_y_t();
            bus.ppu_data.set_nametable_y_v(t);
            let t = bus.ppu_data.get_coarse_y_scroll_t();
            bus.ppu_data.set_coarse_y_scroll_v(t);
        }
    }

    // Start vblank period
    pub fn set_vblank(&mut self) {
        let mut bus = self.bus.borrow_mut();
        // Set nmi
        bus.ppu_data.set_vblank(true);
        bus.ppu_data.nmi_occurred = true;
        if bus.ppu_data.get_nmi_enable() {
            bus.nmi_signal = true;
        }
    }

    // Shift current background patterns
    pub fn apply_shift(&mut self) {
        if self.bus.borrow().ppu_data.get_background_enable() {
            self.background_shift_pattern_low <<= 1;
            self.background_shift_pattern_high <<= 1;

            self.background_shift_attrib_low <<= 1;
            self.background_shift_attrib_high <<= 1;
        }
    }

    // Load current background patterns from next
    pub fn load_shift(&mut self) {
        self.background_shift_pattern_low = (self.background_shift_pattern_low & 0xFF00)
            | (self.background_next_pattern_low as u16);
        self.background_shift_pattern_high = (self.background_shift_pattern_high & 0xFF00)
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
    }

    // Fetch the next tile index from nametable
    pub fn set_background_next_nametable(&mut self) {
        let bus = self.bus.borrow();
        self.background_next_nametable = bus.ppu_read(0x2000 | (bus.ppu_data.vram_addr & 0x0FFF));
    }

    // Fetch the next attributes from nametable attribute table
    pub fn set_background_next_attribute(&mut self) {
        let bus = self.bus.borrow();
        let mut attrib_addr: u16 = 0x23C0;
        attrib_addr |= (bus.ppu_data.get_nametable_y_v() as u16) << 11;
        attrib_addr |= (bus.ppu_data.get_nametable_x_v() as u16) << 10;
        attrib_addr |= ((bus.ppu_data.get_coarse_y_scroll_v() as u16) >> 2) << 3;
        attrib_addr |= (bus.ppu_data.get_coarse_x_scroll_v() as u16) >> 2;
        self.background_next_attrib = bus.ppu_read(attrib_addr);

        if bus.ppu_data.get_coarse_y_scroll_v() & 0x02 != 0 {
            self.background_next_attrib >>= 4;
        }
        if bus.ppu_data.get_coarse_x_scroll_v() & 0x02 != 0 {
            self.background_next_attrib >>= 2;
        }
        self.background_next_attrib &= 0x03;
    }

    // Get the next background pattern low order bits
    pub fn set_background_next_pattern_low(&mut self) {
        let bus = self.bus.borrow();
        let mut pattern_addr: u16 = bus.ppu_data.get_fine_y_scroll_v() as u16;

        pattern_addr += (self.background_next_nametable as u16) << 4;

        if bus.ppu_data.get_background_table_select() {
            pattern_addr += 0x1000;
        }

        self.background_next_pattern_low = bus.ppu_read(pattern_addr);
    }

    // Get the next background pattern high order bits
    pub fn set_background_next_pattern_high(&mut self) {
        let bus = self.bus.borrow();
        let mut pattern_addr: u16 = bus.ppu_data.get_fine_y_scroll_v() as u16;

        pattern_addr += (self.background_next_nametable as u16) << 4;

        if bus.ppu_data.get_background_table_select() {
            pattern_addr += 0x1000;
        }

        pattern_addr += 8;

        self.background_next_pattern_high = bus.ppu_read(pattern_addr);
    }

    // Get the next sprite patterns
    // low : low vs high sprite pattern bits
    pub fn set_sprite_next_pattern(&mut self, low: bool) {
        let bus = self.bus.borrow();
        let current_sprite: usize = ((self.cycle - 256) / 8) as usize;

        // Checks for bad sprite data
        if current_sprite >= self.num_next_sprites_found as usize
        {
            return;
        }

        let mut pattern_addr: u16;

        if !bus.ppu_data.get_sprite_size() {
            // 8x8 sprite mode

            if !bus.ppu_data.get_sprite_table_select() {
                pattern_addr = 0x0000;
            } else {
                pattern_addr = 0x1000;
            }

            // Make sure this is floored
            pattern_addr += (self.next_scanline_sprites[current_sprite][1] as u16) << 4;

            // Handle vertical flip
            if self.next_scanline_sprites[current_sprite][2] & 0x80 == 0 {
                // No flip
                pattern_addr +=
                    (self.scanline as u16) - (self.next_scanline_sprites[current_sprite][0] as u16);
            } else {
                // Flip
                pattern_addr += 7
                    - ((self.scanline as u16)
                        - (self.next_scanline_sprites[current_sprite][0] as u16));
            }
        } else {
            // 8x16 sprite mode

            if self.next_scanline_sprites[current_sprite][1] & 0x01 == 0 {
                pattern_addr = 0x0000;
            } else {
                pattern_addr = 0x1000;
            }

            // Handle vertical flip
            if self.next_scanline_sprites[current_sprite][2] & 0x80 == 0 {
                // No flip

                // Top or bottom tile
                if (self.scanline as u16) - (self.next_scanline_sprites[current_sprite][0] as u16)
                    < 8
                {
                    // Top
                    pattern_addr += (self.next_scanline_sprites[current_sprite][1] as u16) << 4;
                } else {
                    // Bottom
                    pattern_addr +=
                        ((self.next_scanline_sprites[current_sprite][1] as u16) + 1) << 4;
                }
                pattern_addr +=
                    (self.scanline as u16) - (self.next_scanline_sprites[current_sprite][0] as u16);
            } else {
                // Flip

                // Top or bottom tile
                if (self.scanline as u16) - (self.next_scanline_sprites[current_sprite][0] as u16)
                    < 8
                {
                    // Top
                    pattern_addr +=
                        ((self.next_scanline_sprites[current_sprite][1] as u16) + 1) << 4;
                } else {
                    // Bottom
                    pattern_addr += (self.next_scanline_sprites[current_sprite][1] as u16) << 4;
                }

                pattern_addr += 7
                    - ((self.scanline as u16)
                        - (self.next_scanline_sprites[current_sprite][0] as u16));
            }
        }

        if !low {
            pattern_addr += 8;
        }

        let mut tmp_pattern = bus.ppu_read(pattern_addr);

        // Horizontal Flip
        if self.next_scanline_sprites[current_sprite][2] & 0x40 == 0 {
            let mut flipped = 0;
            for i in 0..8 {
                flipped |= ((((tmp_pattern >> i) & 0x01) as u16) << (7 - i)) as u8;
            }
            tmp_pattern = flipped;
        }

        if low {
            self.scanline_sprite_patterns_low[current_sprite] = tmp_pattern;
        } else {
            self.scanline_sprite_patterns_high[current_sprite] = tmp_pattern;
        }
    }

    // Scroll vram pointer horizontally
    pub fn scroll_horizontal(&mut self) {
        let mut bus = self.bus.borrow_mut();
        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            if bus.ppu_data.get_coarse_x_scroll_v() == 31 {
                bus.ppu_data.set_coarse_x_scroll_v(0);

                if bus.ppu_data.get_nametable_x_v() == 0 {
                    bus.ppu_data.set_nametable_x_v(1);
                } else {
                    bus.ppu_data.set_nametable_x_v(0);
                }
            } else {
                let v = bus.ppu_data.get_coarse_x_scroll_v() + 1;
                bus.ppu_data.set_coarse_x_scroll_v(v);
            }
        }
    }

    // Scroll vram pointer vertically
    pub fn scroll_vertical(&mut self) {
        let mut bus = self.bus.borrow_mut();
        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            if bus.ppu_data.get_fine_y_scroll_v() < 7 {
                let v = bus.ppu_data.get_fine_y_scroll_v() + 1;
                bus.ppu_data.set_fine_y_scroll_v(v);
            } else {
                bus.ppu_data.set_fine_y_scroll_v(0);

                if bus.ppu_data.get_coarse_y_scroll_v() == 29 {
                    bus.ppu_data.set_coarse_y_scroll_v(0);

                    if bus.ppu_data.get_nametable_y_v() == 0 {
                        bus.ppu_data.set_nametable_y_v(1);
                    } else {
                        bus.ppu_data.set_nametable_y_v(0);
                    }
                } else if bus.ppu_data.get_coarse_y_scroll_v() == 31 {
                    bus.ppu_data.set_coarse_y_scroll_v(0);
                } else {
                    let v = bus.ppu_data.get_coarse_y_scroll_v() + 1;
                    bus.ppu_data.set_coarse_y_scroll_v(v);
                }
            }
        }
    }

    // Transfer horizontal data from temp vram to main vram
    pub fn transfer_horizontal(&mut self) {
        let mut bus = self.bus.borrow_mut();
        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            let t = bus.ppu_data.get_nametable_x_t();
            bus.ppu_data.set_nametable_x_v(t);
            let t = bus.ppu_data.get_coarse_x_scroll_t();
            bus.ppu_data.set_coarse_x_scroll_v(t);
        }
    }

    // Transfer vertical data from temp vram to main vram
    pub fn transfer_vertical(&mut self) {
        let mut bus = self.bus.borrow_mut();
        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            let t = bus.ppu_data.get_nametable_y_t();
            bus.ppu_data.set_nametable_y_v(t);
            let t = bus.ppu_data.get_coarse_y_scroll_t();
            bus.ppu_data.set_coarse_y_scroll_v(t);
            let t = bus.ppu_data.get_fine_y_scroll_t();
            bus.ppu_data.set_fine_y_scroll_v(t);
        }
    }

    // Actually render a pixel based on internal state setup in clock()
    pub fn render_pixel(&mut self) {
        let mut bus = self.bus.borrow_mut();

        if !bus.ppu_data.get_sprite_left_column_enable() && self.cycle < 8 {
            // Don't render in first 8 pixels if left column not enabled 
            return;
        }

        let mut background_pixel = 0x00;
        let mut background_palette = 0x00;

        let mut sprite_pixel = 0x00;
        let mut sprite_palette = 0x00;
        let mut sprite_priority = 0x00;

        // Get background pixel and attrib 
        if bus.ppu_data.get_background_enable()
            && (bus.ppu_data.get_background_left_column_enable() || self.cycle >= 8)
        {
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

        let mut scanline_sprite_zero = false;

        if bus.ppu_data.get_sprite_enable()
            && (bus.ppu_data.get_sprite_left_column_enable() || self.cycle >= 8)
        {
            // Sprites ordered by priority, so stop searching (but keep shifting) as
            // soon as we fine a pixel
            let mut found_pixel = false;

            // current_scanline_sprites is an 8x4 array with:
            // [Y_pos][tile index][attribute][X_pos (get shifted)]
            for i in 0..8 {
                // Shift counts down to zero
                if self.current_scanline_sprites[i][3] == 0 {
                    if self.scanline_sprite_shift_counters[i] < 8 {
                        if !found_pixel {
                            let mux = ((0x01 as u16)
                                << self.scanline_sprite_shift_counters[i] as u16)
                                as u8;

                            if self.scanline_sprite_patterns_low[i] & mux != 0 {
                                sprite_pixel |= 0x01;
                            }
                            if self.scanline_sprite_patterns_high[i] & mux != 0 {
                                sprite_pixel |= 0x02;
                            }

                            sprite_palette = (self.current_scanline_sprites[i][2] & 0x03) + 4;
                            sprite_priority = (self.current_scanline_sprites[i][2] >> 5) & 0x01;

                            if sprite_pixel != 0 {
                                found_pixel = true;
                                if i == 0 {
                                    scanline_sprite_zero = true;
                                }
                            }
                        }
                        self.scanline_sprite_shift_counters[i] += 1;
                    }
                } else {
                    self.current_scanline_sprites[i][3] -= 1;
                }
            }
        }

        let mut pixel = 0x00;
        let mut palette = 0x00;

        // Background vs sprite priority
        if background_pixel == 0 && sprite_pixel != 0 {
            pixel = sprite_pixel;
            palette = sprite_palette;
        }
        if background_pixel != 0 && sprite_pixel == 0 {
            pixel = background_pixel;
            palette = background_palette;
        }
        if background_pixel != 0 && sprite_pixel != 0 {
            if sprite_priority == 0 {
                pixel = sprite_pixel;
                palette = sprite_palette;
            } else {
                pixel = background_pixel;
                palette = background_palette;
            }

            // Sprite 0 hit
            // this is the scanline with sprite zero, and the first sprite hit
            if self.sprite_zero_on_scanline && scanline_sprite_zero {
                bus.ppu_data.set_sprite_hit(true);
            }
        }

        // Get RGBA data from pallet indexed by pixel
        let true_pixel = PALLET_TO_RGBA
            [(bus.ppu_read(0x3F00 + ((palette as u16) << 2) + (pixel as u16)) & 0x3F) as usize];

        // Offset into screen buffer
        let offset =
            4 * (((self.scanline as u16) as usize) * 256 + (((self.cycle - 1) as u16) as usize));

        // Set RGB in screen buffer
        if offset < 4 * 256 * 240 {
            self.screen[(offset + 0) as usize] = true_pixel.r;
            self.screen[(offset + 1) as usize] = true_pixel.g;
            self.screen[(offset + 2) as usize] = true_pixel.b;
            self.screen[(offset + 3) as usize] = true_pixel.a;
        }
    }

    // The big clock function, drives the ppu
    pub fn clock(&mut self) {
        {
            let mut bus = self.bus.borrow_mut();
            // Check for oam dma
            if bus.oam_dma_ppu {
                for i in 0x0..0x100 {
                    let addr = bus.oam_dma_addr + (i as u16);
                    bus.ppu_data.oam[i] = bus.read(addr, false);
                }
                bus.oam_dma_ppu = false;
            }

            if self.scanline == 0
                && self.cycle == 0
                && !self.even
                && (bus.ppu_data.get_sprite_enable() || bus.ppu_data.get_background_enable())
            {
                // Skip a clock cycle if:
                // * on cycle 0
                // * scanline 0 
                // * odd frame
                // * rendering
                self.cycle += 1;
            }
        }

        // Visible scanlines + pre-render scanline
        if self.scanline < 240 || self.scanline == 261 {
            if self.scanline == 261 {
                // pre-render scanline
                if self.cycle == 1 {
                    self.pre_render_setup();
                }
            }
            // rendering
            if self.cycle == 0 {
                // idle cycle

                // Do some internal setup
                // Set secondary oam to 0xFF
                // and copy next to current
                for i in 0..7 {
                    self.current_scanline_sprites[i] = self.next_scanline_sprites[i];
                    self.next_scanline_sprites[i] = [0xFF, 0xFF, 0xFF, 0xFF];
                    self.scanline_sprite_shift_counters[i] = 0;
                }

                self.num_next_sprites_found = 0;
                self.search_oam_index = 0;
                self.search_current_sprite_byte = -1;
                self.sprite_zero_on_scanline = false;
            } else if self.cycle <= 256 || (self.cycle > 320 && self.cycle <= 336) {
                if self.cycle == 256 {
                    self.scroll_vertical();
                }

                // Backgrounds

                // Shift
                self.apply_shift();
                // current line tile data fetch
                match self.cycle % 8 {
                    0 => {
                        self.scroll_horizontal();
                    }
                    1 => {
                        self.load_shift();
                        self.set_background_next_nametable();
                    }
                    3 => {
                        self.set_background_next_attribute();
                    }
                    5 => {
                        self.set_background_next_pattern_low();
                    }
                    7 => {
                        self.set_background_next_pattern_high();
                    }
                    _ => {}
                }

                // Sprites
                if self.scanline < 240
                    && self.bus.borrow().ppu_data.get_sprite_enable()
                    && self.cycle > 64
                    && self.cycle <= 256
                    && self.search_oam_index < 256
                {

                    // Search for sprites is the oam to render
                    // Reads from oam happen on odd cycles, so we will only work on odd cycles
                    // TODO: Implement sprite overflow bug better
                    if self.num_next_sprites_found < 8 {
                        if self.search_current_sprite_byte == -1 {
                            self.next_scanline_sprites[self.num_next_sprites_found as usize][0] =
                                self.bus.borrow_mut().ppu_data.oam[self.search_oam_index as usize];
                            self.search_oam_index += 1;
                            self.search_current_sprite_byte = 0;
                        } else if self.search_current_sprite_byte == 0 {
                            // If sprite in range
                            let mut sprite_height = 8;
                            if self.bus.borrow().ppu_data.get_sprite_size() {
                                sprite_height = 16;
                            }
                            if self.next_scanline_sprites[self.num_next_sprites_found as usize][0]
                                <= self.scanline as u8
                                && self.next_scanline_sprites[self.num_next_sprites_found as usize]
                                    [0]
                                    + sprite_height
                                    > self.scanline as u8
                            {
                                if self.num_next_sprites_found == 7 {
                                    self.num_next_sprites_found = 8;
                                    self.bus.borrow_mut().ppu_data.set_sprite_overflow(true);
                                } else {
                                    self.num_next_sprites_found += 1;
                                    self.search_current_sprite_byte = 1;

                                    if self.search_oam_index < 4 {
                                        self.sprite_zero_on_scanline = true;
                                    }
                                }
                            } else {
                                self.next_scanline_sprites[self.num_next_sprites_found as usize]
                                    [0] = 0xFF;
                                self.search_current_sprite_byte = -1;
                                self.search_oam_index += 3;
                            }
                        } else if self.search_current_sprite_byte < 4 {
                            self.next_scanline_sprites[self.num_next_sprites_found as usize - 1]
                                [self.search_current_sprite_byte as usize] =
                                self.bus.borrow_mut().ppu_data.oam[self.search_oam_index as usize];
                            self.search_oam_index += 1;
                            self.search_current_sprite_byte += 1;
                        } else {
                            self.search_current_sprite_byte = -1;
                        }
                    }
                }

                if self.scanline < 240 && self.cycle < 256 {
                    self.render_pixel();
                }
            }

            if self.cycle > 256 && self.cycle <= 320 && self.scanline != 261 {
                match self.cycle % 8 {
                    // Low bits
                    5 => {
                        self.set_sprite_next_pattern(true);
                    }
                    // High bits
                    7 => {
                        self.set_sprite_next_pattern(false);
                    }
                    _ => {}
                }
            }

            if self.cycle == 257 {
                self.transfer_horizontal();
            }

            if self.scanline == 261 && self.cycle >= 280 && self.cycle < 305 {
                self.transfer_vertical();
            }
            // fetch two bytes for unknown reason
            if self.cycle == 338 || self.cycle == 340 {
                self.set_background_next_nametable();
            }
        } else if self.scanline == 240 { // post render scanline
        } else if self.scanline <= 260 {
            // vblank
            if self.scanline == 241 && self.cycle == 1 {
                self.set_vblank();
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

                self.render_frame = true;
            }
        }
    }
}
// }}}
