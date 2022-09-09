struct 2A03 {
    a   : u8,  // Accumulator
    x   : u8,  // Register
    y   : u8,  // Register
    pc  : u16, // Program Counter
    stp : u8,  // Stack Pointer 
    stat: u8,  // Status Register

    bus : &Bus // Reference to main bus
}

enum AddressingModes {
    ACC,
    IMD,
    ZPG,
    ABS,
    REL,
    IND,
    ZIX,
    ZIY,
    AIX,
    AIY,
    IIX,
    IIY,
    NUL
}

let addressingModesRefrence: [u8, 0xFF] = []




impl 2A03 {
    // Global var
    let cycle: u8;
    let operand: u8;

    // Setup functions
    fn setup(bus: &Bus) {}

    // Interface functions
    fn clock() {
        if (cycle == 0) {
            let opcode:u8 = read(pc++);
            
            setAddressMode(opcode);
            cycle += execute(opcode);
        }
        cycle--;
    }

    fn reset() {}
    fn irq() {}
    fn nmi() {}

    // Internal functions

    fn read(addr: u16) -> u8 {
        return bus.read();
    }
    fn write(addr: u16, value: u8) {
        bus.write(addr, value);
    }

    
    fn setAddressMode(opcode: u8) {}
    fn execute(opcode: u8) -> u8 {}

}
