// Vim folding 
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::Bus;
use crate::get_asm;


const NMI_VEC: u16 = 0xfffa;
const RESET_VEC: u16 = 0xfffc;
const BRK_VEC: u16 = 0xfffe;

//: Cpu {{{
pub struct Cpu<'a> {
    pub a   : u8,  // Accumulator
    pub x   : u8,  // Register
    pub y   : u8,  // Register
    pub pc  : u16, // Program Counter
    pub stp : u8,  // Stack Pointer 
    pub stat: u8,  // Status Register
    pub cycl: u32, // CPU Ticks 
    pub next: u32, // Tick of next instruction

    pub bus : &'a mut Bus<'a> // Reference to main bus
}
//: }}}

//: Flags {{{
enum Flags {
    CA = 0b00000001, // Carry
    ZE = 0b00000010, // Zero
    ID = 0b00000100, // Interrupt Disable
    DC = 0b00001000, // Decimal 
    B1 = 0b00010000, // B flag bit one
    B2 = 0b00100000, // B flag bit two
    OV = 0b01000000, // Overflow
    NG = 0b10000000, // Negative
}
//: }}}

//: AddrM {{{
pub enum AddrM {
    IMP, // Implicit
    ACC, // Accumulator
    IMD, // Immediate
    ZPG, // Zero Page
    ABS, // Absolute
    ADR, // Fake addressing mode used for debugging
    REL, // Relative
    IND, // Indirect
    ZIX, // Zero Page Indexed X
    ZIY, // Zero Page Indexed Y
    AIX, // Absolute Indexed X
    AIY, // Absolute Indexed Y
    IIX, // Indexed Indirect X
    IIY, // Indirect Indexed Y
    NUL, // Invalid Operation
}
//: }}}

//: addressingModesFull6502 {{{
/*
let addressingModesFull6502: [u8, 0xFF] = [
   IMP, IIX, NUL, NUL, NUL, ZPG, ZPG, NUL, IMP, IMD, ACC, NUL, NUL, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   ABS, IIX, NUL, NUL, ZPG, ZPG, ZPG, NUL, IMP, IMD, ACC, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   IMP, IIX, NUL, NUL, NUL, ZPG, ZPG, NUL, IMP, IMD, ACC, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   IMP, IIX, NUL, NUL, NUL, ZPG, ZPG, NUL, IMP, IMD, ACC, NUL, IND, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   NUL, IIX, NUL, NUL, ZPG, ZPG, ZPG, NUL, IMP, NUL, IMP, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, ZIX, ZIX, ZIY, NUL, IMP, AIY, IMP, NUL, NUL, AIX, NUL, NUL,
   IMD, IIX, IMD, NUL, ZPG, ZPG, ZPG, NUL, IMP, IMD, IMP, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, ZIX, ZIX, ZIY, NUL, IMP, AIY, IMP, NUL, AIX, AIX, AIY, NUL,
   IMD, IIX, NUL, NUL, ZPG, ZPG, ZPG, NUL, IMP, IMD, IMP, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   IMD, IIX, NUL, NUL, ZPG, ZPG, ZPG, NUL, IMP, IMD, IMP, NUL, ABS, ABS, ABS, NUL,
   REL, IIY, NUL, NUL, NUL, ZIX, ZIX, NUL, IMP, AIY, NUL, NUL, NUL, AIX, AIX, NUL,
   
]*/
//: }}}

//: ADDRESSING_MODE_LOOKUP {{{
pub static ADDRESSING_MODE_LOOKUP: [AddrM; 0x100] = [
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::IMD, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::ADR, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::IMD, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::IMD, AddrM::ADR, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::IMD, AddrM::IND, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::NUL, AddrM::IIX, AddrM::NUL, AddrM::IIX, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::IMP, AddrM::NUL, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::ABS,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::ZIY, AddrM::ZIY, AddrM::IMP, AddrM::AIY, AddrM::IMP, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::NUL, AddrM::NUL,
   AddrM::IMD, AddrM::IIX, AddrM::IMD, AddrM::IIX, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::ABS,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::IIY, AddrM::ZIX, AddrM::ZIX, AddrM::ZIY, AddrM::ZIY, AddrM::IMP, AddrM::AIY, AddrM::IMP, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::AIY, AddrM::AIY,
   AddrM::IMD, AddrM::IIX, AddrM::NUL, AddrM::IIX, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::IMD, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::ABS,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::IIY, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::ZIX, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::AIX, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::AIY,
   AddrM::IMD, AddrM::IIX, AddrM::NUL, AddrM::IIX, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::ABS,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::IIY, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::ZIX, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::AIY, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::AIX,
];
//: }}}


// PAGE_BOUNDARY_ADDITION
const PBA: u8 = 0x80;
// BRANCH_ADDITION
const BA: u8 = 0x40;

//: CYCLE_COUNTS {{{
static CYCLE_COUNTS: [u8; 0x100] = [ 
    7    , 6    , 0    , 0    , 0    , 3    , 5    , 0    , 3    , 2    , 2    , 2    , 0    , 4    , 6    , 0    ,
    2|BA , 5|PBA, 0    , 0    , 0    , 4    , 6    , 0    , 2    , 4|PBA, 0    , 0    , 0    , 4|PBA, 7    , 0    ,
    6    , 6    , 0    , 0    , 3    , 3    , 5    , 0    , 4    , 2    , 2    , 2    , 4    , 4    , 6    , 0    ,
    2|BA , 5|PBA, 0    , 0    , 0    , 4    , 6    , 0    , 2    , 4|PBA, 0    , 0    , 4    , 4|PBA, 7    , 0    ,
    6    , 6    , 0    , 0    , 0    , 3    , 5    , 0    , 3    , 2    , 2    , 2    , 3    , 4    , 6    , 0    ,
    2|BA , 5|PBA, 0    , 0    , 0    , 4    , 6    , 0    , 2    , 4|PBA, 0    , 0    , 0    , 4|PBA, 7    , 0    ,
    6    , 6    , 0    , 0    , 0    , 3    , 5    , 0    , 4    , 2    , 2    , 2    , 5    , 4    , 6    , 0    ,
    2|BA , 5|PBA, 0    , 0    , 0    , 4    , 6    , 0    , 2    , 4|PBA, 0    , 0    , 0    , 4|PBA, 7    , 0    ,
    0    , 6    , 0    , 6    , 3    , 3    , 3    , 3    , 2    , 0    , 2    , 0    , 4    , 4    , 4    , 4    ,
    2|BA , 6    , 2    , 0    , 4    , 4    , 4    , 4    , 2    , 5    , 2    , 0    , 4    , 5    , 4    , 0    ,
    2    , 6    , 2    , 6    , 3    , 3    , 3    , 3    , 2    , 2    , 2    , 0    , 4    , 4    , 4    , 4    ,
    2|BA , 5|PBA, 0    , 5    , 4    , 4    , 4    , 4    , 2    , 4|PBA, 2    , 0    , 4|PBA, 4|PBA, 4|PBA, 4    ,
    2    , 6    , 0    , 8    , 3    , 3    , 5    , 5    , 2    , 2    , 2    , 2    , 4    , 4    , 6    , 6    ,
    2|BA , 5|PBA, 0    , 8    , 3    , 4    , 6    , 6    , 2    , 4|PBA, 2    , 7    , 4    , 4|PBA, 7    , 7    ,
    2    , 6    , 0    , 8    , 3    , 3    , 5    , 5    , 2    , 2    , 2    , 0    , 4    , 4    , 6    , 6    ,
    2|BA , 5|PBA, 0    , 8    , 0    , 4    , 6    , 6    , 2    , 4|PBA, 2    , 7    , 0    , 4|PBA, 7    , 7    ,
];
//: }}}

// const addressingModesRefrence: [u8, 0xFF] = []

//: CPU_DEBUG {{{
impl<'a> std::fmt::Debug for Cpu<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
          .field("a", &self.a)
          .field("x", &self.x)
          .field("y", &self.y)
          .field("pc", &self.pc)
          .field("stp", &self.stp)
          .field("stat", &self.stat)
          .field("cycl", &self.cycl)
          .field("next", &self.next)
          .finish()
    }
}
//: }}}

//: CPU {{{
impl<'a> Cpu<'a> {
    // Setup functions
    pub fn new(bus: &'a mut Bus<'a>) -> Self {
        Self { 
            a: 0u8,
            x: 0u8,
            y: 0u8,
            pc: 0x8000,
            stp: 0xFD,
            //stat: 0x34,
            stat: 0x24,
            cycl: 0u32,
            next: 0u32,
            bus: bus
        }
    }

    // Interface functions
    pub fn clock(&mut self) {
        if self.cycl == self.next {
            //print_asm(self.bus, self.pc);
            //C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
            //println!("PC:0x{:04x},A:0x{:02x},X:0x{:02x},Y:0x{:02x},STAT:0b{:b},STP:0x{:02x},CYCL:{}", self.pc, self.a, self.x, self.y, self.stat, self.stp, self.cycl);
            //print_debug_string(self.bus);
            //C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
            print!("{:04X}  ", self.pc);
            match ADDRESSING_MODE_LOOKUP[self.read(self.pc) as usize] {
                AddrM::ACC|AddrM::IMP => { // One Byte
                    print!("{:02X}       ", self.read(self.pc));
                }
                AddrM::IMD|AddrM::ZPG|AddrM::REL|AddrM::ZIX|AddrM::ZIY|AddrM::IIX|AddrM::IIY => { // Two Bytes 
                    print!("{:02X} {:02X}    ", self.read(self.pc), self.read(self.pc+1));
                }
                AddrM::ABS|AddrM::ADR|AddrM::AIX|AddrM::AIY|AddrM::IND => { // Three Bytes
                    print!("{:02X} {:02X} {:02X} ", self.read(self.pc), self.read(self.pc+1), self.read(self.pc+2));
                }
                AddrM::NUL => {
                    print!("INVLD: {:02X}", self.read(self.pc));
                }

            }
            print!(" {}  ", get_asm(self));
            println!{"A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:>3},{:>3} CYC:{}", self.a, self.x, self.y, self.stat, self.stp, 0, 0, self.cycl};
            // for i in 0..0xFF {
            //     print!{"{}|", self.bus.read(0x100 + i)}; 
            // }
            // println!("");
            
            let opcode:u8 = self.read(self.pc);
            self.pc += 1;
           

            let opcode_cycles:u8 = CYCLE_COUNTS[opcode as usize] & 0x0F;
            let (real_address, mut cycle_addition) = self.set_address_mode(opcode);
            cycle_addition += self.execute(opcode, real_address);
            self.next = self.cycl + (opcode_cycles as u32) + (cycle_addition as u32); 
        }
        //self.cycl -= 1;
        self.cycl += 1;
    }

    pub fn reset() {}
    pub fn irq() {}
    pub fn nmi() {}

    // Internal functions
    fn set_flag(&mut self, flag: Flags, value: bool) {
        let bit: u8 = flag as u8;
        if value {
            self.stat |= bit;
        } else {
            self.stat &= 0xFF - bit;
        }
    }

    fn get_flag(&self, flag: Flags) -> u8 {
        let bit: u8 = flag as u8;
        let value: u8 = self.stat & bit;
        if value == 0 {
            return 0;
        } else {
            return 1;
        }
    }

    // Reads an address in memory
    fn read(&self, addr: u16) -> u8 {
        return self.bus.read(addr);
    }

    fn read_word_little(&self, addr: u16) -> u16 {
        return self.bus.read_word_little(addr);
    }
    
    fn read_word_little_wrap(&self, addr: u16) -> u16 {
        return self.bus.read_word_little_wrap(addr);
    }
    // Writes a value to memory
    fn write(&mut self, addr: u16, value: u8) {
        self.bus.write(addr, value);
    }

    //: set_address_mode {{{
    fn set_address_mode(&mut self, opcode: u8) -> (u16, u8) {
        let check_for_page_boundary:bool = (CYCLE_COUNTS[opcode as usize]& PBA) != 0x00;

        let mut real_address: u16;
        let mut cycle_addition: u8 = 0;
        match ADDRESSING_MODE_LOOKUP[opcode as usize] {
            AddrM::ABS|AddrM::ADR => {
                real_address = self.read_word_little(self.pc);

                self.pc += 2;
            }
            AddrM::AIX => {
                let low_byte: u8 = self.read(self.pc);
                let high_byte: u8 = self.read(self.pc+1);
                real_address = ((high_byte as u16) << 8) + low_byte as u16;

                real_address = real_address.wrapping_add(self.x as u16);

                if check_for_page_boundary && (real_address & 0xFF) < (self.x as u16) { 
                    cycle_addition += 1;
                }

                self.pc += 2;
            }
            AddrM::AIY => {
                let low_byte: u8 = self.read(self.pc);
                let high_byte: u8 = self.read(self.pc+1);
                real_address = ((high_byte as u16) << 8) + low_byte as u16;

                real_address = real_address.wrapping_add(self.y as u16);

                if check_for_page_boundary && (real_address & 0xFF) < (self.y as u16) { 
                    cycle_addition += 1;
                }

                self.pc += 2;
            }
            AddrM::IMD => {
               real_address = self.pc;
               self.pc += 1;
            }
            AddrM::IND => {
                let low_byte: u8 = self.read(self.pc);
                let high_byte: u8 = self.read(self.pc+1);
                let effective_address: u16 = ((high_byte as u16) << 8).wrapping_add(low_byte as u16);

                real_address = self.read_word_little_wrap(effective_address);

                self.pc += 2;
            }
            AddrM::IIX => {
                let loc: u8 = self.read(self.pc);
                let low_byte: u8 = self.read(loc.wrapping_add(self.x) as u16);
                let high_byte: u8 = self.read((loc.wrapping_add(self.x)).wrapping_add(1) as u16);
                real_address = ((high_byte as u16) << 8) + low_byte as u16;
                //real_address = self.read_word_little(effective_address);

                self.pc += 1;
            }
            AddrM::IIY => {
                let loc: u8 = self.read(self.pc);
                let low_byte: u8 = self.read(loc as u16);
                let high_byte: u8 = self.read(loc.wrapping_add(1) as u16);
                let effective_address: u16 = ((high_byte as u16) << 8) + low_byte as u16;

                real_address = effective_address.wrapping_add(self.y as u16);

                if check_for_page_boundary && (real_address & 0xFF) < (self.y as u16) { 
                    cycle_addition += 1;
                }

                self.pc += 1;
            }
            AddrM::REL => {
               real_address = self.pc;
               self.pc += 1;
            }
            AddrM::ZPG => {
                real_address = self.read(self.pc) as u16;
                self.pc += 1;
            }
            AddrM::ZIX => {
                real_address = (self.read(self.pc) as u16).wrapping_add(self.x as u16);
                real_address &= 0xFF;
                self.pc += 1;
            }
            AddrM::ZIY => {
                real_address = (self.read(self.pc) as u16).wrapping_add(self.y as u16);
                real_address &= 0xFF;
                self.pc += 1;
            }
            _ => { // ACC / IMP 
                return (0,0);
            }
        }
        return (real_address, cycle_addition);
    }
    //: }}}

    //: execute helpers {{{
    fn branch(&mut self, real_address: u16) -> u8 {
        let old_pc = self.pc;
        self.pc = (self.pc as i32).wrapping_add(self.read(real_address) as i32) as u16; 
        //self.pc = (self.pc as i32 + self.read(real_address) as i32) as u16; 
        
        if self.pc & 0xFF00 != old_pc & 0xFF00 {
            return 3; // Page Boundry + Branch Taken
        } else {
            return 1; // Branch Taken
        }
    } 
    //: }}}

    //: execute {{{
    // Given an opcode, finds the amount of consecutive bits in memory to read, 
    fn execute(&mut self, opcode: u8, real_address: u16) -> u8 {
        let mut cycle_addition = 0;

        // Note opcodes with a * are unofficial
        match opcode {
            0x0B|0x2B => { // *AAC (And And Copy)
                self.a &= self.read(real_address);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
                
                self.set_flag(Flags::CA, self.get_flag(Flags::NG) != 0);
            }
            0x69|0x65|0x75|0x6D|0x7D|0x79|0x61|0x71 => { // ADC (Add With Carry)
                let tmp:u16 = (self.a as u16).wrapping_add(self.read(real_address) as u16).wrapping_add(self.get_flag(Flags::CA) as u16);
                
                // Overflow flag, I probably messed this up 
                self.set_flag(Flags::OV, (((self.a ^ self.read(real_address)) & 0x80 == 0)) && ((self.a ^ tmp as u8) & 0x80 == 0x80));
                
                self.a = tmp as u8;

                self.set_flag(Flags::CA, tmp > 0xFF);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 

            }
            0x29|0x25|0x35|0x2D|0x3D|0x39|0x21|0x31 => { // AND (Logical AND)
                self.a &= self.read(real_address);
            
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
            }
            0x0A => { // ASL (Shift Left One Bit) Accumulator
                self.set_flag(Flags::CA, (self.a & 0x80) != 0);
                
                self.a = self.a << 1;

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
            }
            0x06|0x16|0x0E|0x1E => { // ASL (Shift Left One Bit) 
                let mut operand = self.read(real_address);
                self.set_flag(Flags::CA, (operand & 0x80) != 0);

                operand = operand << 1;

                self.set_flag(Flags::ZE, operand == 0x00);
                self.set_flag(Flags::NG, (operand & 0x80) != 0);

                self.write(real_address, operand);
            }
            0x4B => { // *ASR (And + Shift Right)
                self.a &= self.read(real_address);
                self.a = self.a >> 1;

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); // Nintendulator is clever and just sets it to zero, but not me.
            }
            0x6B => { // *ARR (And + Rotate)
                self.a &= self.read(real_address);

                let high_bit: u8 = self.get_flag(Flags::CA);
                self.a = (self.a >> 1) + (high_bit << 7);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
                
                self.set_flag(Flags::CA, ((self.a >> 6) & 1) != 0);
                self.set_flag(Flags::OV, self.get_flag(Flags::CA) ^ ((self.a >> 5) & 1) != 0);
            }
            0xCB => { // *AXS (A and X Subtract)
                // I'm not sure about this, ask ronan
                let tmp: u16 = ((self.a & self.x) as u16).wrapping_sub(self.read(real_address) as u16);
                self.x = tmp as u8;

                self.set_flag(Flags::CA, tmp <= 0xFF);
                self.set_flag(Flags::ZE, self.x == 0x00);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0); 
            }
            0x90 => { // BCC (Branch if Carry Clear)
                if self.get_flag(Flags::CA) == 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0xB0 => { // BCS (Branch if Carry set)
                if self.get_flag(Flags::CA) != 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0xF0 => { // BEQ (Branch if Equal)
                if self.get_flag(Flags::ZE) != 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0x24|0x2C => { // BIT (Bit test)
                // if zero flag is clear
                self.set_flag(Flags::ZE, self.a & self.read(real_address) == 0); 
                self.set_flag(Flags::OV, self.read(real_address) & 0x70 != 0);
                self.set_flag(Flags::NG, self.read(real_address) & 0x80 != 0);
            }
            0x30 => { // BMI (Branch if Minus)
                if self.get_flag(Flags::NG) != 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0xD0 => { // BNE (Branch if Not Equal)
                // If zero flag is clear
                if self.get_flag(Flags::ZE) == 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0x10 => { // BPL (Branch on Plus)
                if self.get_flag(Flags::NG) == 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0x00 => { // BRK (Force Interrupt)
                self.write(0x0100 + self.stp as u16, (self.pc >> 8) as u8);
                self.stp = self.stp.wrapping_sub(1);
                
                self.write(0x0100 + self.stp as u16, self.pc as u8);
                self.stp = self.stp.wrapping_sub(1);


                self.write(0x0100 + self.stp as u16, self.stat);
                self.stp = self.stp.wrapping_sub(1);
                self.set_flag(Flags::ID, true);
                self.pc = BRK_VEC;
            }
            0x50 => { // BVC (Branch if Overflow Clear)
                if self.get_flag(Flags::OV) == 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0x70 => { // BVS (Branch if Overflowe set)
                if self.get_flag(Flags::OV) != 0 {
                    cycle_addition += self.branch(real_address);
                }
            }
            0x18 => { // CLC (Clear Carry Flag)
                self.set_flag(Flags::CA, false);
            }
            0xD8 => { // CLD (Clear Decimal Mode)
                self.set_flag(Flags::DC, false);
            }
            0x58 => { // CLI (Clear Interrupt Disable)
                self.set_flag(Flags::ID, false);
            }
            0xB8 => { // CLV (Clear Overflow Flag)
                self.set_flag(Flags::OV, false);
            }
            0xC9|0xC5|0xD5|0xCD|0xDD|0xD9|0xC1|0xD1 => { // CMP (Compare)
                let m: u8 = self.read(real_address);
                let res: u8 = self.a.wrapping_sub(m);
                self.set_flag(Flags::CA, self.a >= m);
                self.set_flag(Flags::ZE, self.a == m);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xE0|0xE4|0xEC => { // CPX (Compare X register)
                let m = self.read(real_address);
                let res: u8 = self.x.wrapping_sub(m);
                self.set_flag(Flags::CA, self.x >= m);
                self.set_flag(Flags::ZE, self.x == m);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xC0|0xC4|0xCC => { // CPY (Compare Y register)
                let m = self.read(real_address);
                let res: u8 = self.y.wrapping_sub(m);
                self.set_flag(Flags::CA, self.y >= m);
                self.set_flag(Flags::ZE, self.y == m);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xC3|0xC7|0xCF|0xD3|0xD7|0xDB|0xDF => { // *DCP (Decrement + Compare) 
                let m: u8 = self.read(real_address);
                let tmp_res: u8 = m.wrapping_sub(1);
                self.write(real_address, tmp_res);

                let res: u8 = self.a.wrapping_sub(tmp_res);
                self.set_flag(Flags::CA, self.a >= res);
                self.set_flag(Flags::ZE, self.a == res);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xC6|0xD6|0xCE|0xDE => { // DEC (Decrement Memory)
                let m: u8 = self.read(real_address);
                let res: u8 = m.wrapping_sub(1);

                // TODO: Check this over. I'm not sure if this is correct
                self.write(real_address, res);

                self.set_flag(Flags::ZE, res == 0);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xCA => { // DEX (Decrement X Register)
                self.x = self.x.wrapping_sub(1);

                self.set_flag(Flags::ZE, self.x == 0);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0);
            }
            0x88 => { // DEY (Decrement Y Register)
                self.y = self.y.wrapping_sub(1);

                self.set_flag(Flags::ZE, self.y == 0);
                self.set_flag(Flags::NG, (self.y & 0x80) != 0);
            }
            0x49|0x45|0x55|0x4D|0x5D|0x59|0x41|0x51 => { // EOR (Exclusive OR)
                let m: u8 = self.read(real_address);

                self.a ^= m;
                self.set_flag(Flags::ZE, self.a == 0);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
            }
            0xE6|0xF6|0xEE|0xFE => { // INC (Increment Memory)
                let m: u8 = self.read(real_address);
                let res: u8 = m.wrapping_add(1);

                self.write(real_address, res);

                self.set_flag(Flags::ZE, res == 0);
                self.set_flag(Flags::NG, (res & 0x80) != 0);
            }
            0xE8 => { // INX (Increment X Register)
                self.x = self.x.wrapping_add(1);

                self.set_flag(Flags::ZE, self.x == 0);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0);
            }
            0xC8 => { // INY (Increment Y Register)
                self.y = self.y.wrapping_add(1);

                self.set_flag(Flags::ZE, self.y == 0);
                self.set_flag(Flags::NG, (self.y & 0x80) != 0);
            }

            0xE3|0xE7|0xEF|0xF3|0xF7|0xFB|0xFF => { // ISB (Increment + Subtract)
                let m: u8 = self.read(real_address);
                let res: u8 = m.wrapping_add(1);

                self.write(real_address, res);

                let tmp:u16 = (self.a as u16).wrapping_sub(res as u16).wrapping_sub((1 as u16).wrapping_sub(self.get_flag(Flags::CA) as u16));
                
                // Overflow flag, I probably messed this up 
                //self.set_flag(Flags::OV, (((self.a ^ self.read(real_address)) & 0x80 == 0)) && ((self.a ^ tmp as u8) & 0x80 == 0x80));
                self.set_flag(Flags::OV, (((self.a as u16) ^ tmp) & ((!self.read(real_address) as u16) ^ tmp) & 0x80) != 0x00);
                
                self.a = tmp as u8;
            
                self.set_flag(Flags::CA, tmp <= 0xFF);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x4C|0x6C => { // JMP (Jump)
                           // TODO: This function needs work
                           //  "An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00. This is fixed in some later chips like the 65SC02 so for compatibility always ensure the indirect vector is not at the end of the page."
                           // -- https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP

                           //let operand = self.read(real_address);
                           //self.pc = self.read_word_little(real_address);
                self.pc = real_address;
            }
            0x20 => { // JSR (Jump to Subroutine)
                let return_point: u16 = self.pc - 1; 
                self.write(0x100 + self.stp as u16, (return_point >> 8) as u8);
                self.stp = self.stp.wrapping_sub(1);
                self.write(0x100 + self.stp as u16, return_point as u8);
                self.stp = self.stp.wrapping_sub(1);
                //self.pc = self.read_word_little(real_address);
                self.pc = real_address;
            }

            0xA3|0xA7|0xAF|0xB3|0xB7|0xBF => { // LAX (Load Accumulator and X)
                self.a = self.read(real_address);
                self.x = self.a;

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }

            0xA9|0xA5|0xB5|0xAD|0xBD|0xB9|0xA1|0xB1 => { // LDA (Load Accumulator) 
                self.a = self.read(real_address);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }

            0xA2|0xA6|0xB6|0xAE|0xBE => { // LDX (Load X)
                self.x = self.read(real_address);
                self.set_flag(Flags::ZE, self.x == 0x00);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0); 
            }

            0xA0|0xA4|0xB4|0xAC|0xBC => { // LDY (Load Y)
                self.y = self.read(real_address);
                self.set_flag(Flags::ZE, self.y == 0x00);
                self.set_flag(Flags::NG, (self.y & 0x80) != 0); 
            }
            
            0x4A => { // LSR (Logical Shift Right) for Accumulator
                self.set_flag(Flags::CA, (self.a & 0x01) != 0);
                
                self.a = self.a >> 1;

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
               
            }
            0x46|0x56|0x4E|0x5E => { // LSR (Logical Shift Right) for Memory
                let mut operand = self.read(real_address);

                self.set_flag(Flags::CA, (operand & 0x01) != 0);
                
                operand = operand >> 1;

                self.write(real_address, operand);

                self.set_flag(Flags::ZE, operand == 0x00);
                self.set_flag(Flags::NG, (operand & 0x80) != 0);
                
            }
            0xEA => { // NOP (No Operation)

            }
            0x09|0x05|0x15|0x0D|0x1D|0x19|0x01|0x11 => { // ORA (Or Memory with Accumulator)
                self.a |= self.read(real_address);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x48 => { // PHA (Push Accumulator)
                self.write(0x100 + self.stp as u16, self.a);
                self.stp = self.stp.wrapping_sub(1);
            }
            0x08 => { // PHP (Push Processer Status)
                self.write(0x100 + self.stp as u16, self.stat | (Flags::B1 as u8));
                self.stp = self.stp.wrapping_sub(1);
            }
            0x68 => { // PLA (Pull Accumulator)
                self.stp = self.stp.wrapping_add(1);
                self.a = self.read(0x100 + self.stp as u16);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x28 => { // PLP (Pull Processer Status)
                self.stp = self.stp.wrapping_add(1);
                self.stat = self.read(0x0100 + self.stp as u16) & 0b11101111 | (Flags::B2 as u8); // B flag
            }
            0x26|0x36|0x2E|0x3E => { // ROL (Rotate Left)
                let low_bit: u8 = self.get_flag(Flags::CA);
                self.set_flag(Flags::CA, (self.read(real_address) & 0x80) != 0);

                let tmp: u8 = (self.read(real_address) << 1) + low_bit;
                self.write(real_address, tmp);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (tmp & 0x80) != 0); 
            }
            0x2A => { // ROL (Rotate Left) for accumulator 
                let low_bit: u8 = self.get_flag(Flags::CA);
                self.set_flag(Flags::CA, (self.a & 0x80) != 0);

                self.a = (self.a << 1) + low_bit;

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }

            0x66|0x76|0x6E|0x7E => { // ROR (Rotate Right)
                let high_bit: u8 = self.get_flag(Flags::CA);
                self.set_flag(Flags::CA, (self.read(real_address) & 0x01) != 0);

                let tmp: u8 = (self.read(real_address) >> 1) + (high_bit << 7);
                self.write(real_address, tmp);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (tmp & 0x80) != 0); 
            }
            0x6A => { // ROR (Rotate Right) for accumulator 
                let high_bit: u8 = self.get_flag(Flags::CA);
                self.set_flag(Flags::CA, (self.a & 0x01) != 0);

                self.a = (self.a >> 1) + (high_bit << 7);

                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }

            0x40 => { // RTI (Return from interrupt)
                self.stp = self.stp.wrapping_add(1);
                self.stat = self.read(0x0100 + self.stp as u16) & 0b11101111 | (Flags::B2 as u8); // B flag

                self.stp = self.stp.wrapping_add(1);
                let stack_one = self.read(0x0100 + self.stp as u16);

                self.stp = self.stp.wrapping_add(1);
                let stack_two = self.read(0x0100 + self.stp as u16);

                self.pc = ((stack_two as u16) << 8) + stack_one as u16;
            }
            0x60 => { // RTS (Return from subroutine)
                self.stp = self.stp.wrapping_add(1);
                let stack_one = self.read(0x0100 + self.stp as u16);
                self.stp = self.stp.wrapping_add(1);
                let stack_two = self.read(0x0100 + self.stp as u16);
                
                self.pc = ((stack_two as u16) << 8) + stack_one as u16 + 1;
            }
            0x83|0x87|0x8F|0x97 => { // *SAX (Store A and X)
                self.write(real_address, self.a & self.x);
            }
            0xE9|0xE5|0xF5|0xED|0xFD|0xF9|0xE1|0xF1 => { // SBC (Subtract with carry)
                let tmp:u16 = (self.a as u16).wrapping_sub(self.read(real_address) as u16).wrapping_sub((1 as u16).wrapping_sub(self.get_flag(Flags::CA) as u16));
                
                // Overflow flag, I probably messed this up 
                //self.set_flag(Flags::OV, (((self.a ^ self.read(real_address)) & 0x80 == 0)) && ((self.a ^ tmp as u8) & 0x80 == 0x80));
                self.set_flag(Flags::OV, (((self.a as u16) ^ tmp) & ((!self.read(real_address) as u16) ^ tmp) & 0x80) != 0x00);
                
                self.a = tmp as u8;
            
                self.set_flag(Flags::CA, tmp <= 0xFF);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x38 => { // SEC (Set Carry)
                self.set_flag(Flags::CA, true);
            }
            0xF8 => { // SED (Set Decimal)
                self.set_flag(Flags::DC, true);
            }
            0x78 => { // SEI (Set Interrupt)
                self.set_flag(Flags::ID, true);
            }
            0x85|0x95|0x8D|0x9D|0x99|0x81|0x91 => { // STA (Store A)
                self.write(real_address, self.a);
            }
            0x86|0x96|0x8E => { // STX (Store X)
                self.write(real_address, self.x);
            }
            0x84|0x94|0x8C => { // STY (Store Y)
                self.write(real_address, self.y);
            }
            0xAA => { // TAX (Transfer A to X)
                self.x = self.a;
                self.set_flag(Flags::ZE, self.x == 0x00);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0); 
            }
            0xA8 => { // TAY (Transfer A to Y)
                self.y = self.a;
                self.set_flag(Flags::ZE, self.y == 0x00);
                self.set_flag(Flags::NG, (self.y & 0x80) != 0); 
            }
            0xBA => { // TSX (Transfer stack pointer to X)
                self.x = self.stp;
                self.set_flag(Flags::ZE, self.x == 0x00);
                self.set_flag(Flags::NG, (self.x & 0x80) != 0); 
            }
            0x8A => { // TXA (Transfer X to A)
                self.a = self.x;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x9A => { // TXS (Transfer X to stack pointer)
                self.stp = self.x;
            }
            0x98 => { // TYA (Transfer Y to A)
                self.a = self.y;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            _ => {
                return 2; // Treat as nop
            }
        }
        return cycle_addition;
    }
    //: }}}
}
//: }}}
