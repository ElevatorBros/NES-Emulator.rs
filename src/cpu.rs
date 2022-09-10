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
    CA = 0b00000001, // Carry
    ZE = 0b00000010, // Zero
    ID = 0b00000100, // Interrupt Disable
    DC = 0b00001000, // Decimal 
    B1 = 0b00010000, // B flag bit one
    B2 = 0b00100000, // B flag bit two
    OV = 0b01000000, // Overflow
    NG = 0b10000000, // Negative
}

enum AddrM {
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
    pub fn clock(&self) {
        if self.cycl == 0 {
            let opcode:u8 = self.read(self.pc);
            self.pc += 1;
            
            let operand = self.set_address_mode(opcode);
            self.cycl += self.execute(opcode, operand);
        }
        self.cycl -= 1;
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

    // Reads an address in memory
    fn read(&self, addr: u16) -> u8 {
        return self.bus.read(addr);
    }
    // Writes a value to memory
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

    // Given an opcode, finds the amount of consecutive bits in memory to read, 
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

        return opcode_cycles;
    }

}
