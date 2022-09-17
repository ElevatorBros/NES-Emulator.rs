use crate::AddrM;
use crate::ADDRESSING_MODE_LOOKUP;
use crate::Bus;


static ASM_LOOKUP : [&str; 0x100] = [
    "BRK", "ORA", "NUL", "NUL", "NUL", "ORA", "ASL", "NUL", "PHP", "ORA", "ASL", "NUL", "NUL", "ORA", "ASL", "NUL",
    "BPL", "ORA", "NUL", "NUL", "NUL", "ORA", "ASL", "NUL", "CLC", "ORA", "NUL", "NUL", "NUL", "ORA", "ASL", "NUL",
    "JSR", "AND", "NUL", "NUL", "BIT", "AND", "ROL", "NUL", "PLP", "AND", "ROL", "NUL", "BIT", "AND", "ROL", "NUL",
    "BMI", "AND", "NUL", "NUL", "NUL", "AND", "ROL", "NUL", "SEC", "AND", "NUL", "NUL", "NUL", "AND", "ROL", "NUL",
    "RTI", "EOR", "NUL", "NUL", "NUL", "EOR", "LSR", "NUL", "PHA", "EOR", "LSR", "NUL", "JMP", "EOR", "LSR", "NUL",
    "BVC", "EOR", "NUL", "NUL", "NUL", "EOR", "LSR", "NUL", "CLI", "EOR", "NUL", "NUL", "NUL", "EOR", "LSR", "NUL",
    "RTS", "ADC", "NUL", "NUL", "NUL", "ADC", "ROR", "NUL", "PLA", "ADC", "ROR", "NUL", "JMP", "ADC", "ROR", "NUL",
    "BVS", "ADC", "NUL", "NUL", "NUL", "ADC", "ROR", "NUL", "SEI", "ADC", "NUL", "NUL", "NUL", "ADC", "ROR", "NUL",
    "NUL", "STA", "NUL", "NUL", "STY", "STA", "STX", "NUL", "DEY", "NUL", "TXA", "NUL", "STY", "STA", "STX", "NUL",
    "BCC", "STA", "NUL", "NUL", "STY", "STA", "STX", "NUL", "TYA", "STA", "TXS", "NUL", "NUL", "STA", "NUL", "NUL",
    "LDY", "LDA", "LDX", "NUL", "LDY", "LDA", "LDX", "NUL", "TAY", "LDA", "TAX", "NUL", "LDY", "LDA", "LDX", "NUL",
    "BCS", "LDA", "NUL", "NUL", "LDY", "LDA", "LDX", "NUL", "CLV", "LDA", "TSX", "NUL", "LDY", "LDA", "LDX", "NUL",
    "CPY", "CMP", "NUL", "NUL", "CPY", "CMP", "DEC", "NUL", "INY", "CMP", "DEX", "NUL", "CPY", "CMP", "DEC", "NUL",
    "BNE", "CMP", "NUL", "NUL", "NUL", "CMP", "DEC", "NUL", "CLD", "CMP", "NUL", "NUL", "NUL", "CMP", "DEC", "NUL",
    "CPX", "SBC", "NUL", "NUL", "CPX", "SBC", "INC", "NUL", "INX", "SBC", "NOP", "NUL", "CPX", "SBC", "INC", "NUL",
    "BEQ", "SBC", "NUL", "NUL", "NUL", "SBC", "INC", "NUL", "SED", "SBC", "NUL", "NUL", "NUL", "SBC", "INC", "NUL",
];


pub fn print_asm(bus: &Bus, pc: u16) {
    let opcode: u8 = bus.read(pc);
    print!("{}", ASM_LOOKUP[opcode as usize]);
    print!(" ");
    match ADDRESSING_MODE_LOOKUP[opcode as usize] {
        AddrM::ACC => {
            print!("A");
        }
        AddrM::ABS => {
            print!("${:X}", bus.read_word_little(pc+1));
        }
        AddrM::AIX => {
            print!("${:X},X", bus.read_word_little(pc+1));
        }
        AddrM::AIY => {
            print!("${:X},Y", bus.read_word_little(pc+1));
        }
        AddrM::IMD => {
            print!("#${:X}", bus.read(pc+1));
        }
        AddrM::IMP => {
            print!("");
        }
        AddrM::IND => {
            print!("$({:X})", bus.read_word_little(pc+1));
        } 
        AddrM::IIX => {
            print!("$({:X},X)", bus.read(pc+1));
        }
        AddrM::IIY => {
            print!("$({:X}),Y", bus.read(pc+1));
        }
        AddrM::REL | AddrM::ZPG => {
            print!("${:X}", bus.read(pc+1));
        }
        AddrM::ZIX => {
            print!("${:X},X", bus.read(pc+1));
        }
        AddrM::ZIY => {
            print!("${:X},Y", bus.read(pc+1));
        }
        AddrM::NUL => {
            print!("Invalid Opcode");
        }
    }
    print!("\n");
}
