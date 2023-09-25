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
    pub vram_addr: u16,
    pub vram_addr_tmp: u16,
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

struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

//: Ppu Functions {{{
impl Ppu {
    pub fn new() -> Self {
        Self {
            //chr_rom: [0; 0x2000],
            //vram: [0; 0x800],
            //pallet: [0; 0x100],
            //data: [0; 0x4000],
            vram_addr: 0,
            vram_addr_tmp: 0,

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
    fn get_background_tile_select(&self, bus: &mut Bus) -> bool {
        (bus.ppu_data.ctrl & (1 << 4)) != 0
    }
    // 0: $0000; 1: $1000
    fn get_sprite_tile_select(&self, bus: &mut Bus) -> bool {
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
            // Pre render scanline stuff

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
                        //shift

                        // Nametable
                        self.background_next_nametable =
                            bus.ppu_read(0x2000 + (bus.ppu_data.vram_addr & 0x0FFF));
                    }
                    3 => { // Attribute
                    }
                    5 => { // Pattern low
                    }
                    7 => { // Pattern high
                    }
                    _ => {}
                }

                if self.background_next_nametable != 0 && self.background_next_nametable != 0x24 {
                    self.put_pixel(
                        self.scanline as u16,
                        self.cycle as u16,
                        RGBA {
                            r: 0xff,
                            g: 0xff,
                            b: 0xff,
                            a: 0xff,
                        },
                    );
                } else {
                    self.put_pixel(
                        self.scanline as u16,
                        self.cycle as u16,
                        RGBA {
                            r: 0x00,
                            g: 0x00,
                            b: 0x00,
                            a: 0x00,
                        },
                    );
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
