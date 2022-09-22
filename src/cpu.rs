use crate::Bus;
use crate::print_asm;

pub struct Cpu<'a> {
    a   : u8,  // Accumulator
    x   : u8,  // Register
    y   : u8,  // Register
    pc  : u16, // Program Counter
    stp : u8,  // Stack Pointer 
    stat: u8,  // Status Register
    cycl: u8,  // CPU Ticks remaining

    bus : &'a mut Bus<'a> // Reference to main bus
}

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

pub enum AddrM {
    IMP, // Implicit
    ACC, // Accumulator
    IMD, // Immediate
    ZPG, // Zero Page
    ABS, // Absolute
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

pub static ADDRESSING_MODE_LOOKUP: [AddrM; 0x100] = [
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::NUL, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::ABS, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::IMP, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::ACC, AddrM::NUL, AddrM::IND, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::NUL, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::NUL, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::ZIY, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::IMP, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::NUL, AddrM::NUL,
   AddrM::IMD, AddrM::IIX, AddrM::IMD, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::ZIY, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::IMP, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::AIY, AddrM::NUL,
   AddrM::IMD, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
   AddrM::IMD, AddrM::IIX, AddrM::NUL, AddrM::NUL, AddrM::ZPG, AddrM::ZPG, AddrM::ZPG, AddrM::NUL, AddrM::IMP, AddrM::IMD, AddrM::IMP, AddrM::NUL, AddrM::ABS, AddrM::ABS, AddrM::ABS, AddrM::NUL,
   AddrM::REL, AddrM::IIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::ZIX, AddrM::ZIX, AddrM::NUL, AddrM::IMP, AddrM::AIY, AddrM::NUL, AddrM::NUL, AddrM::NUL, AddrM::AIX, AddrM::AIX, AddrM::NUL,
];


static CYCLE_COUNTS: [u8; 0x100] = [
    7, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 0, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 6, 0,
    6, 5, 0, 0, 3, 3, 5, 0, 5, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 4, 4, 6, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 3, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 6, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 4, 2, 2, 0, 6, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 6, 0,
    0, 6, 0, 0, 3, 3, 3, 0, 2, 0, 2, 0, 4, 4, 4, 0,
    2, 6, 2, 0, 3, 3, 3, 0, 2, 2, 2, 0, 4, 4, 4, 0,
    2, 5, 0, 0, 4, 4, 4, 0, 2, 4, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 2, 0, 0, 4, 7, 0,
];

// const addressingModesRefrence: [u8, 0xFF] = []


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
          .finish()
    }
}


impl<'a> Cpu<'a> {
    // Setup functions
    pub fn new(bus: &'a mut Bus<'a>) -> Self {
        Self { 
            a: 0u8,
            x: 0u8,
            y: 0u8,
            pc: 0x8000,
            stp: 0u8,
            stat: 0u8,
            cycl: 0u8,
            bus: bus
        }
    }

    // Interface functions
    pub fn clock(&mut self) {
        if self.cycl == 0 {
            print_asm(self.bus, self.pc);
            let opcode:u8 = self.read(self.pc);
            self.pc += 1;
            
            let (operand, raw_operand, second_raw_operand, cycle_addition) = self.set_address_mode(opcode);
            self.cycl += cycle_addition;
            self.cycl += self.execute(opcode, operand, raw_operand, second_raw_operand);
        }
        self.cycl -= 1;
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
    // Writes a value to memory
    fn write(&mut self, addr: u16, value: u8) {
        self.bus.write(addr, value);
    }

    fn set_address_mode(&mut self, opcode: u8) -> (u8, u8, u8, u8) {
        let mut operand: u8 = 0;
        let mut raw_operand: u8 = 0;
        let mut second_raw_operand: u8 = 0;
        let mut cycle_addition: u8 = 0;
        match ADDRESSING_MODE_LOOKUP[opcode as usize] {
            AddrM::ACC => {
               operand = self.a;
            }
            AddrM::ABS => {
                raw_operand = self.read(self.pc);
                second_raw_operand = self.read(self.pc+1);

                operand = self.read(self.read_word_little(self.pc));
                self.pc += 2;
            }
            AddrM::IMD|AddrM::REL => {
               let raw_operand = self.read(self.pc);
               self.pc += 1;
               operand = raw_operand;
            }
            _ => {
                return (0,0,0,0);
            }
        }
        return (operand, raw_operand, second_raw_operand, cycle_addition);
    }

    // Given an opcode, finds the amount of consecutive bits in memory to read, 
    fn execute(&mut self, opcode: u8, operand: u8, raw_operand: u8, second_raw_operand: u8) -> u8 {
        let opcode_cycles = CYCLE_COUNTS[opcode as usize];

        match opcode {
            0x69|0x65|0x75|0x6D|0x7D|0x79|0x61|0x71 => { // ADC (Add With Carry)
                let tmp:u16 = self.a as u16 + operand as u16 + self.get_flag(Flags::CA) as u16;
                
                // Overflow flag, I probably messed this up 
                self.set_flag(Flags::OV, (((self.a ^ operand) & 0x80 == 0)) && ((self.a ^ tmp as u8) & 0x80 == 0x80));
                
                self.a = tmp as u8;

                self.set_flag(Flags::CA, tmp > 0xFF);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 

            }
            0x29|0x25|0x35|0x2D|0x39|0x21|0x31 => { // AND (Logical AND)
                self.a &= operand;
            
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
            }
            0x0A|0x06|0x16|0x0E|0x1E => { // ASL (Shift Left One Bit)
                self.a = self.a << 1;

                self.set_flag(Flags::CA, (self.a & 0x80) != 0);
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0);
            }
            0xA9|0xA5|0xB5|0xAD|0xBD|0xB9|0xA1|0xB1 => { // LDA (Load Accumulator)
                self.a = operand;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x38 => { // SEC (Set Carry)
                self.set_flag(Flags::CA, true);
            }
            0xFD => { // SED (Set Decimal)
                self.set_flag(Flags::DC, true);
            }
            0x78 => { // SEI (Set Interrupt)
                self.set_flag(Flags::ID, true);
            }
            0x85|0x95|0x8D|0x9D|0x99|0x81|0x91 => { // STA (Store A)
                self.write(raw_operand as u16 + ((second_raw_operand as u16) << 8), self.a)
            }
            0x44|0x96|0x8E => { // STX (Store X)
                self.write(raw_operand as u16 + ((second_raw_operand as u16) << 8), self.x)
            }
            0x84|0x94|0x8C => { // STY (Store Y)
                self.write(raw_operand as u16 + ((second_raw_operand as u16) << 8), self.y)
            }
            0xAA => { // TAX (Transfer A to X)
                self.x = self.a;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0xA8 => { // TAY (Transfer A to Y)
                self.y = self.a;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x8A => { // TXA (Transfer X to A)
                self.a = self.x;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            0x9A => { // TXS (Transfer X to stack pointer)
                self.stp = self.x;
            }
            0xBA => { // TSX (Transfer stack pointer to X)
                self.stp = self.x;
            }
            0x98 => { // TYA (Transfer Y to A)
                self.a = self.y;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, (self.a & 0x80) != 0); 
            }
            _ => {
                return 0;
            }
        }
        return opcode_cycles;
    }
}
