// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::bus::*;
use std::cell::RefCell;
use std::rc::Rc;

//: SpriteData {{{
#[derive(Copy, Clone)]
pub struct SpriteData {
    pub pattern_shift: u8,
    pub latch: u8,
    pub counter: u8,
}
//: }}}

//: Ppu {{{
pub struct Ppu<'a> {
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

    pub bus: Rc<RefCell<Bus<'a>>>, // Reference to main bus
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
// }}}

//: PpuData {{{
pub struct PpuData {
    pub nmi_occurred: bool,

    pub ctrl: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scroll_latch: bool,
    pub addr_latch: bool,
    pub data: u8,
    pub data_buffer: u8,

    pub fine_x_scroll: u8,
    pub vram_addr: u16,
    pub temp_vram_addr: u16,
}
// }}}

//: PpuData Functions {{{
impl PpuData {
    // Vram addr functions
    pub fn get_fine_y_scroll_v(&self) -> u8 {
        ((self.vram_addr >> 12) & 0x0007) as u8
    }
    pub fn get_fine_y_scroll_t(&self) -> u8 {
        ((self.temp_vram_addr >> 12) & 0x0007) as u8
    }
    pub fn get_nametable_x_v(&self) -> u8 {
        ((self.vram_addr >> 11) & 0x0001) as u8
    }
    pub fn get_nametable_x_t(&self) -> u8 {
        ((self.temp_vram_addr >> 11) & 0x0001) as u8
    }
    pub fn get_nametable_y_v(&self) -> u8 {
        ((self.vram_addr >> 10) & 0x0001) as u8
    }
    pub fn get_nametable_y_t(&self) -> u8 {
        ((self.temp_vram_addr >> 10) & 0x0001) as u8
    }
    pub fn get_coarse_y_scroll_v(&self) -> u8 {
        ((self.vram_addr >> 5) & 0x001F) as u8
    }
    pub fn get_coarse_y_scroll_t(&self) -> u8 {
        ((self.temp_vram_addr >> 5) & 0x001F) as u8
    }
    pub fn get_coarse_x_scroll_v(&self) -> u8 {
        (self.vram_addr & 0x001F) as u8
    }
    pub fn get_coarse_x_scroll_t(&self) -> u8 {
        (self.temp_vram_addr & 0x001F) as u8
    }

    pub fn set_fine_y_scroll_v(&mut self, value: u8) {
        self.vram_addr = (self.vram_addr & 0x0FFF) | ((value as u16) << 12);
    }
    pub fn set_fine_y_scroll_t(&mut self, value: u8) {
        self.temp_vram_addr = (self.temp_vram_addr & 0x0FFF) | ((value as u16) << 12);
    }
    pub fn set_nametable_x_v(&mut self, value: u8) {
        self.vram_addr = (self.vram_addr & 0x77FF) | ((value as u16) << 11);
    }
    pub fn set_nametable_x_t(&mut self, value: u8) {
        self.temp_vram_addr = (self.temp_vram_addr & 0x77FF) | ((value as u16) << 11);
    }
    pub fn set_nametable_y_v(&mut self, value: u8) {
        self.vram_addr = (self.vram_addr & 0x7BFF) | ((value as u16) << 10);
    }
    pub fn set_nametable_y_t(&mut self, value: u8) {
        self.temp_vram_addr = (self.temp_vram_addr & 0x7BFF) | ((value as u16) << 10);
    }
    pub fn set_coarse_y_scroll_v(&mut self, value: u8) {
        self.vram_addr = (self.vram_addr & 0x7C1F) | ((value as u16) << 5);
    }
    pub fn set_coarse_y_scroll_t(&mut self, value: u8) {
        self.temp_vram_addr = (self.temp_vram_addr & 0x7C1F) | ((value as u16) << 5);
    }
    pub fn set_coarse_x_scroll_v(&mut self, value: u8) {
        self.vram_addr = (self.vram_addr & 0x7FE0) | (value as u16);
    }
    pub fn set_coarse_x_scroll_t(&mut self, value: u8) {
        self.temp_vram_addr = (self.temp_vram_addr & 0x7FE0) | (value as u16);
    }

    // Register functions
    /* CPU Registers */
    // PPU_CTRL
    // 0: disable, 1: enable
    fn get_nmi_enable(&self) -> bool {
        (self.ctrl & (1 << 7)) != 0
    }
    // 0: slave, 1: master
    fn get_master_slave(&self) -> bool {
        (self.ctrl & (1 << 6)) != 0
    }
    // 0: 8x8, 1: 8x16
    fn get_sprite_size(&self) -> bool {
        (self.ctrl & (1 << 5)) != 0
    }
    // 0: $0000; 1: $1000
    fn get_background_table_select(&self) -> bool {
        (self.ctrl & (1 << 4)) != 0
    }
    // 0: $0000; 1: $1000
    fn get_sprite_table_select(&self) -> bool {
        (self.ctrl & (1 << 3)) != 0
    }
    // 0: horizontal; 1: vertical
    fn get_increment_mode(&self) -> bool {
        (self.ctrl & (1 << 2)) != 0
    }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_base_nametable_addr(&self) -> u8 {
        self.ctrl & 0x04
    }

    // PPU_MASK
    // 0: color, 1: greyscale
    fn get_greyscale(&self) -> bool {
        (self.mask & (1 << 0)) != 0
    }
    // 0: hide; 1: show
    fn get_background_left_column_enable(&self) -> bool {
        (self.mask & (1 << 2)) != 0
    }
    // 0: hide, 1: show sprites in leftmost 8 pixels of screen
    fn get_sprite_left_column_enable(&self) -> bool {
        (self.mask & (1 << 1)) != 0
    }
    // 0: hide; 1: show
    fn get_background_enable(&self) -> bool {
        (self.mask & (1 << 4)) != 0
    }
    // 0: hide, 1: show background in leftmost 8 pixels of screen
    fn get_sprite_enable(&self) -> bool {
        (self.mask & (1 << 3)) != 0
    }
    // 0: none; 1: emphasize
    fn get_emphasize_red(&self) -> bool {
        (self.mask & (1 << 5)) != 0
    }
    // 0: none; 1: emphasize
    fn get_emphasize_green(&self) -> bool {
        (self.mask & (1 << 6)) != 0
    }
    // 0: $2000, 1: $2400, 2: $2800, 3:$2C00
    fn get_emphasize_blue(&self) -> bool {
        (self.mask & (1 << 7)) != 0
    }

    // PPU_STATUS
    // Open bus is weird, make sure to come back to this
    // Only write to the low five bits
    fn set_open_bus(&mut self, value: u8) {
        self.status = (value & 0x1F) | (self.status & 0xE0)
    }
    // Weird as well because of hardware bug, look into sprite evaluation
    fn set_sprite_overflow(&mut self, value: bool) {
        if value {
            self.status = self.status | 0x20;
        } else {
            self.status = self.status & 0xDF;
        }
    }
    fn set_sprite_hit(&mut self, value: bool) {
        if value {
            self.status = self.status | 0x40;
        } else {
            self.status = self.status & 0xBF;
        }
    }
    fn set_vblank(&mut self, value: bool) {
        if value {
            self.status = self.status | 0x80;
        } else {
            self.status = self.status & 0x7F;
        }
    }

    // OAM_ADDR
    fn get_oam_addr(&self) -> u8 {
        self.oam_addr
    }

    // OAM_DATA
    fn get_oam_data(&self) -> u8 {
        self.oam_data
    }
    fn set_oam_data(&mut self, value: u8) {
        self.oam_data = value
    }

    // PPU_DATA
    fn get_ppu_data(&self) -> u8 {
        self.data
    }
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

            primary_oam: [0; 0x100], // 256 bytes internal oam
            secondary_oam: [0; 0x20],
            current_sprite_data: [SpriteData {
                pattern_shift: 0,
                latch: 0,
                counter: 0,
            }; 8],

            scanline: 0,
            cycle: 0,
            even: true,

            render_frame: false,
            screen: [0x00; 4 * 256 * 240],
            bus,
        }
    }

    // OAM_DMA
    /*
    fn copy_to_oam(&mut self) {
        let bus = self.bus.borrow_mut();
        for i in 0x0..0x100 {
            self.primary_oam[i] = bus.read(bus.oam_dma_addr + (i as u16), false);
        }
        bus.oam_dma_ppu = false;
    }
    */

    /*
    fn get_rgba(&mut self, pixel: u8, pallet: u8, bus: RefCell<Bus>) -> RGBA {
        let addr = 0x3F00 + ((pallet as u16) << 2) + (pixel as u16);
        PALLET_TO_RGBA[(bus.borrow_mut().ppu_read(addr) & 0x3F) as usize]
    }
    */

    /*
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
    */

    pub fn pre_render_setup(&mut self) {
        let mut bus = self.bus.borrow_mut();
        bus.ppu_data.nmi_occurred = false;
        bus.ppu_data.set_vblank(false);
        bus.ppu_data.set_sprite_hit(false);
        bus.ppu_data.set_sprite_overflow(false);

        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            let t = bus.ppu_data.get_fine_y_scroll_t();
            bus.ppu_data.set_fine_y_scroll_v(t);
            let t = bus.ppu_data.get_nametable_y_t();
            bus.ppu_data.set_nametable_y_v(t);
            let t = bus.ppu_data.get_coarse_y_scroll_t();
            bus.ppu_data.set_coarse_y_scroll_v(t);
        }
    }

    pub fn set_vblank(&mut self) {
        let mut bus = self.bus.borrow_mut();
        // Set nmi
        bus.ppu_data.set_vblank(true);
        bus.ppu_data.nmi_occurred = true;
        if bus.ppu_data.get_nmi_enable() {
            bus.nmi_signal = true;
        }
    }

    pub fn apply_shift(&mut self) {
        if self.bus.borrow().ppu_data.get_background_enable() {
            self.background_shift_pattern_low <<= 1;
            self.background_shift_pattern_high <<= 1;

            self.background_shift_attrib_low <<= 1;
            self.background_shift_attrib_high <<= 1;
        }
    }

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

    pub fn set_next_nametable(&mut self) {
        let bus = self.bus.borrow();
        self.background_next_nametable = bus.ppu_read(0x2000 | (bus.ppu_data.vram_addr & 0x0FFF));
    }

    pub fn set_next_attribute(&mut self) {
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

    pub fn set_next_pattern_low(&mut self) {
        let bus = self.bus.borrow();
        let mut pattern_addr: u16 = bus.ppu_data.get_fine_y_scroll_v() as u16;

        pattern_addr += (self.background_next_nametable as u16) << 4;

        if bus.ppu_data.get_background_table_select() {
            pattern_addr += 0x1000;
        }

        self.background_next_pattern_low = bus.ppu_read(pattern_addr);
    }

    pub fn set_next_pattern_high(&mut self) {
        let bus = self.bus.borrow();
        let mut pattern_addr: u16 = bus.ppu_data.get_fine_y_scroll_v() as u16;

        pattern_addr += (self.background_next_nametable as u16) << 4;

        if bus.ppu_data.get_background_table_select() {
            pattern_addr += 0x1000;
        }

        pattern_addr += 8;

        self.background_next_pattern_high = bus.ppu_read(pattern_addr);
    }

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

    pub fn transfer_horizontal(&mut self) {
        let mut bus = self.bus.borrow_mut();
        if bus.ppu_data.get_background_enable() || bus.ppu_data.get_sprite_enable() {
            let t = bus.ppu_data.get_nametable_x_t();
            bus.ppu_data.set_nametable_x_v(t);
            let t = bus.ppu_data.get_coarse_x_scroll_t();
            bus.ppu_data.set_coarse_x_scroll_v(t);
        }
    }

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

    pub fn render_pixel(&mut self) {
        let bus = self.bus.borrow();
        let mut background_pixel = 0x00;
        let mut background_palette = 0x00;

        // Doing the pixel stuff
        if bus.ppu_data.get_background_enable() {
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

        let true_pixel = PALLET_TO_RGBA[(bus
            .ppu_read(0x3F00 + ((background_palette as u16) << 2) + (background_pixel as u16))
            & 0x3F) as usize];

        let offset =
            4 * (((self.scanline as u16) as usize) * 256 + (((self.cycle - 1) as u16) as usize));

        if offset < 4 * 256 * 240 {
            self.screen[(offset + 0) as usize] = true_pixel.r;
            self.screen[(offset + 1) as usize] = true_pixel.g;
            self.screen[(offset + 2) as usize] = true_pixel.b;
            self.screen[(offset + 3) as usize] = true_pixel.a;
        }
    }

    pub fn clock(&mut self) {
        {
            let mut bus = self.bus.borrow_mut();
            // Check for oam dma
            if bus.oam_dma_ppu {
                for i in 0x0..0x100 {
                    let addr = bus.oam_dma_addr + (i as u16);
                    self.primary_oam[i] = bus.read(addr, false);
                }
                bus.oam_dma_ppu = false;
            }

            if self.scanline == 0
                && self.cycle == 0
                && !self.even
                && (bus.ppu_data.get_sprite_enable() || bus.ppu_data.get_background_enable())
            {
                // Skip a clock cycle on cycle 0 scanline 0 if we are on an even frame and rendering
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
            if self.cycle == 0 { // idle cycle
            } else if self.cycle <= 256 || (self.cycle > 320 && self.cycle <= 336) {
                // Shift
                self.apply_shift();

                if self.cycle == 256 {
                    self.scroll_vertical();
                }

                // current line tile data fetch
                match self.cycle % 8 {
                    0 => {
                        self.scroll_horizontal();
                    }
                    1 => {
                        self.load_shift();
                        self.set_next_nametable();
                    }
                    3 => {
                        self.set_next_attribute();
                    }
                    5 => {
                        self.set_next_pattern_low();
                    }
                    7 => {
                        self.set_next_pattern_high();
                    }
                    _ => {}
                }

                if self.scanline < 240 && self.cycle <= 256 {
                    self.render_pixel();
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
                self.set_next_nametable();
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
