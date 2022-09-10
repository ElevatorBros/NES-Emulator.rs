struct Cpu {
    a   : u8,  // Accumulator
    x   : u8,  // Register
    y   : u8,  // Register
    pc  : u16, // Program Counter
    stp : u8,  // Stack Pointer 
    stat: u8,  // Status Register
    cycl: u8,  // CPU Ticks remaining

    bus : &Bus // Reference to main bus
}

enum Flags {
    CA = 0b00000001,   // Carry
    ZE = 0b00000010,   // Zero
    ID = 0b00000100,   // Interrupt Disable
    DC = 0b00001000,   // Decimal 
    B1 = 0b00010000,  // B flag bit one
    B2 = 0b00100000,  // B flag bit two
    OV = 0b01000000,  // Overflow
    NG = 0b10000000, // Negative
}

enum AddressingModes {
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
    NUL, // Invalide Operation
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

const ADDRESSING_MODE_LOOKUP: [u8; 0xFF] = [
   AddressingModes::IMP, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::ACC, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
   AddressingModes::ABS, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::ACC, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
   AddressingModes::IMP, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::ACC, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
   AddressingModes::IMP, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::ACC, AddressingModes::NUL, AddressingModes::IND, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
   AddressingModes::NUL, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::ZIY, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::NUL, AddressingModes::NUL,
   AddressingModes::IMD, AddressingModes::IIX, AddressingModes::IMD, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::ZIY, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::AIY, AddressingModes::NUL,
   AddressingModes::IMD, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
   AddressingModes::IMD, AddressingModes::IIX, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::ZPG, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::IMD, AddressingModes::IMP, AddressingModes::NUL, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::ABS, AddressingModes::NUL,
   AddressingModes::REL, AddressingModes::IIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::ZIX, AddressingModes::ZIX, AddressingModes::NUL, AddressingModes::IMP, AddressingModes::AIY, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::NUL, AddressingModes::AIX, AddressingModes::AIX, AddressingModes::NUL,
];


const CYCLE_COUNTS: [u8; 0xFF] = [
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
];

// const addressingModesRefrence: [u8, 0xFF] = []


impl Cpu {
    // Setup functions
    pub fn new(bus: &Bus) -> Self {
        Self { 
            0, 0, 0, 0x8000, 0, 0, 0, bus
        }
    }

    // Interface functions
    pub fn clock(&self) {
        if self.cycle == 0 {
            let opcode:u8 = self.read(self.pc);
            self.pc += 1;
            
            let operand = self.set_address_mode(opcode);
            self.cycl += self.execute(opcode, operand);
        }
        self.cycle -= 1;
    }

    pub fn reset() {}
    pub fn irq() {}
    pub fn nmi() {}

    // Internal functions
    fn set_flag(&self, bit: u8, value: bool) {
        if value {
            self.stat |= bit;
        } else {
            self.stat &= 0xFF - bit;
        }
    }

    fn read(&self, addr: u16) -> u8 {
        return self.bus.read(addr);
    }
    fn write(&self, addr: u16, value: u8) {
        self.bus.write(addr, value);
    }

    
    fn set_address_mode(&self, opcode: u8) {
        match ADDRESSING_MODE_LOOKUP[opcode] {
            IMD => {
               return self.read(self.pc);
               self.pc += 1;
            }
        }
    }

    fn execute(&self, opcode: u8, operand: u8) -> u8 {
        let opcode = self.read(self.pc);
        self.pc += 1;
        let opcode_cycles = CYCLE_COUNTS[opcode];

        match opcode {
            0xA9|0xA5|0xB5|0xAD|0xBD|0xB9|0xA1|0xB1 => { // LDA (Load Accumulator)
                self.a = operand;
                self.set_flag(Flags::ZE, self.a == 0x00);
                self.set_flag(Flags::NG, self.a & 0x80); 
            }
        }

        return  opcodeCycles;
    }

}
