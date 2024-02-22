// Vim folding
// vim:foldmethod=marker
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bus::Bus;
use crate::cpu::AddrM;
use crate::cpu::Cpu;
use crate::cpu::ADDRESSING_MODE_LOOKUP;
use std::cell::RefMut;

//: ASM_LOOKUP {{{
static ASM_LOOKUP: [&str; 0x100] = [
    " BRK", " ORA", "*KIL", "*SLO", "*IGN", " ORA", " ASL", "*SLO", " PHP", " ORA", " ASL", "*AAC",
    "*IGN", " ORA", " ASL", "*SLO", " BPL", " ORA", "*KIL", "*SLO", "*IGN", " ORA", " ASL", "*SLO",
    " CLC", " ORA", "*NOP", "*SLO", "*IGN", " ORA", " ASL", "*SLO", " JSR", " AND", "*KIL", "*RLA",
    " BIT", " AND", " ROL", "*RLA", " PLP", " AND", " ROL", "*AAC", " BIT", " AND", " ROL", "*RLA",
    " BMI", " AND", "*KIL", "*RLA", "*IGN", " AND", " ROL", "*RLA", " SEC", " AND", "*NOP", "*RLA",
    "*IGN", " AND", " ROL", "*RLA", " RTI", " EOR", "*KIL", "*SRE", "*IGN", " EOR", " LSR", "*SRE",
    " PHA", " EOR", " LSR", "*ASR", " JMP", " EOR", " LSR", "*SRE", " BVC", " EOR", "*KIL", "*SRE",
    "*IGN", " EOR", " LSR", "*SRE", " CLI", " EOR", "*NOP", "*SRE", "*IGN", " EOR", " LSR", "*SRE",
    " RTS", " ADC", "*KIL", "*RRA", "*IGN", " ADC", " ROR", "*RRA", " PLA", " ADC", " ROR", "*ARR",
    " JMP", " ADC", " ROR", "*RRA", " BVS", " ADC", "*KIL", "*RRA", "*IGN", " ADC", " ROR", "*RRA",
    " SEI", " ADC", "*NOP", "*RRA", "*IGN", " ADC", " ROR", "*RRA", "*SKB", " STA", "*SKB", "*SAX",
    " STY", " STA", " STX", "*SAX", " DEY", "*SKB", " TXA", " NUL", " STY", " STA", " STX", "*SAX",
    " BCC", " STA", "*KIL", " NUL", " STY", " STA", " STX", "*SAX", " TYA", " STA", " TXS", " NUL",
    " NUL", " STA", " NUL", " NUL", " LDY", " LDA", " LDX", "*LAX", " LDY", " LDA", " LDX", "*LAX",
    " TAY", " LDA", " TAX", " NUL", " LDY", " LDA", " LDX", "*LAX", " BCS", " LDA", "*KIL", "*LAX",
    " LDY", " LDA", " LDX", "*LAX", " CLV", " LDA", " TSX", " NUL", " LDY", " LDA", " LDX", "*LAX",
    " CPY", " CMP", "*SKB", "*DCP", " CPY", " CMP", " DEC", "*DCP", " INY", " CMP", " DEX", "*AXS",
    " CPY", " CMP", " DEC", "*DCP", " BNE", " CMP", "*KIL", "*DCP", "*IGN", " CMP", " DEC", "*DCP",
    " CLD", " CMP", "*NOP", "*DCP", "*IGN", " CMP", " DEC", "*DCP", " CPX", " SBC", "*SKB", "*ISB",
    " CPX", " SBC", " INC", "*ISB", " INX", " SBC", " NOP", "*SBC", " CPX", " SBC", " INC", "*ISB",
    " BEQ", " SBC", "*KIL", "*ISB", "*IGN", " SBC", " INC", "*ISB", " SED", " SBC", "*NOP", "*ISB",
    "*IGN", " SBC", " INC", "*ISB",
];
//: }}}

//: get_asm {{{
pub fn get_asm(cpu: &Cpu, mut bus: RefMut<Bus<'_>>) -> String {
    let mut asm_string: String;
    let opcode: u8 = bus.read(cpu.pc, true);

    // Assembling cases for each addressing mode
    // Pulls from ASM_LOOKUP for instruction mnemonic, * is unofficial

    asm_string = format!("{}", ASM_LOOKUP[opcode as usize]);
    match ADDRESSING_MODE_LOOKUP[opcode as usize] {
        AddrM::ACC => {
            asm_string = format!("{} A                         ", asm_string);
        }
        AddrM::ABS => {
            let operand: u16 = bus.read_word_little(cpu.pc + 1, true);
            asm_string = format!(
                "{} ${:04X} = {:02X}                ",
                asm_string,
                operand,
                bus.read(operand, true)
            );
        }
        AddrM::ADR => {
            let operand: u16 = bus.read_word_little(cpu.pc + 1, true);
            asm_string = format!("{} ${:04X}                     ", asm_string, operand);
        }
        AddrM::AIX => {
            let operand: u16 = bus.read_word_little(cpu.pc + 1, true);
            let effective_address: u16 = operand + cpu.x as u16;
            asm_string = format!(
                "{} ${:04X},X @ {:04X} = {:02X}       ",
                asm_string,
                operand,
                effective_address,
                bus.read(effective_address, true)
            );
        }
        AddrM::AIY => {
            let operand: u16 = bus.read_word_little(cpu.pc + 1, true);
            let effective_address: u16 = operand.wrapping_add(cpu.y as u16);
            asm_string = format!(
                "{} ${:04X},Y @ {:04X} = {:02X}       ",
                asm_string,
                operand,
                effective_address,
                bus.read(effective_address, true)
            );
        }
        AddrM::IMD => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            asm_string = format!("{} #${:02X}                      ", asm_string, operand);
        }
        AddrM::IMP => {
            asm_string = format!("{}                           ", asm_string);
        }
        AddrM::IND => {
            let operand: u16 = bus.read_word_little(cpu.pc + 1, true);
            let effective_address: u16 = bus.read_word_little_wrap(operand, true);
            asm_string = format!(
                "{} (${:04X}) = {:04X}            ",
                asm_string, operand, effective_address
            );
        }
        AddrM::IIX => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            let mid_address: u8 = operand.wrapping_add(cpu.x);
            let low_byte: u8 = bus.read(mid_address as u16, true);
            let high_byte: u8 =
                bus.read((operand.wrapping_add(cpu.x)).wrapping_add(1) as u16, true);
            let effective_address: u16 = ((high_byte as u16) << 8) + low_byte as u16;

            asm_string = format!(
                "{} (${:02X},X) @ {:02X} = {:04X} = {:02X}  ",
                asm_string,
                operand,
                mid_address,
                effective_address,
                bus.read(effective_address, true)
            );
        }
        AddrM::IIY => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            let low_byte: u8 = bus.read(operand as u16, true);
            let high_byte: u8 = bus.read(operand.wrapping_add(1) as u16, true);
            let raw_address: u16 = ((high_byte as u16) << 8) + low_byte as u16;

            let effective_address: u16 = raw_address.wrapping_add(cpu.y as u16);

            asm_string = format!(
                "{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                asm_string,
                operand,
                raw_address,
                effective_address,
                bus.read(effective_address, true)
            );
        }
        AddrM::REL => {
            let mut offset: u8 = bus.read(cpu.pc + 1, true);

            let effective_address: u16;
            if offset <= 0x7F {
                effective_address = cpu.pc.wrapping_add(offset as u16);
            } else {
                offset = !offset;
                offset += 1;
                effective_address = cpu.pc.wrapping_sub(offset as u16);
            }

            // Plus 2 because the the program counter will be incremented twice before the jump actually happens
            asm_string = format!(
                "{} ${:04X}                     ",
                asm_string,
                effective_address + 2
            );
        }
        AddrM::ZPG => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            asm_string = format!(
                "{} ${:02X} = {:02X}                  ",
                asm_string,
                operand,
                bus.read(operand as u16, true)
            );
        }
        AddrM::ZIX => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            let effective_address: u8 = operand.wrapping_add(cpu.x);
            asm_string = format!(
                "{} ${:02X},X @ {:02X} = {:02X}           ",
                asm_string,
                operand,
                effective_address,
                bus.read(effective_address as u16, true)
            );
        }
        AddrM::ZIY => {
            let operand: u8 = bus.read(cpu.pc + 1, true);
            let effective_address: u8 = operand.wrapping_add(cpu.y);
            asm_string = format!(
                "{} ${:02X},Y @ {:02X} = {:02X}           ",
                asm_string,
                operand,
                effective_address,
                bus.read(effective_address as u16, true)
            );
        }
        AddrM::NUL => {
            asm_string = format!("Invalid Opcode                ");
        }
    }
    asm_string
}
//: }}}

//: output_debug_info {{{
// CPU debug info, prints current instruction and internal state
pub fn output_debug_info(cpu: &Cpu) {
    let mut bus = cpu.bus.borrow_mut();
    // Program counter
    print!("{:04X}  ", cpu.pc);
    // Print out the hex of the current instruction
    match ADDRESSING_MODE_LOOKUP[bus.read(cpu.pc, true) as usize] {
        AddrM::ACC | AddrM::IMP => {
            // One Byte
            print!("{:02X}       ", bus.read(cpu.pc, true));
        }
        AddrM::IMD | AddrM::ZPG | AddrM::REL | AddrM::ZIX | AddrM::ZIY | AddrM::IIX | AddrM::IIY => {
            // Two Bytes
            print!(
                "{:02X} {:02X}    ",
                bus.read(cpu.pc, true),
                bus.read(cpu.pc + 1, true)
            );
        }
        AddrM::ABS | AddrM::ADR | AddrM::AIX | AddrM::AIY | AddrM::IND => {
            // Three Bytes
            print!(
                "{:02X} {:02X} {:02X} ",
                bus.read(cpu.pc, true),
                bus.read(cpu.pc + 1, true),
                bus.read(cpu.pc + 2, true)
            );
        }
        AddrM::NUL => {
            print!("INVLD: {:02X}", bus.read(cpu.pc, true));
        }
    }
    // Print assembled instruction
    print!("{}  ", get_asm(cpu, bus));
    // Print status registers and such
    println! {"A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}", cpu.a, cpu.x, cpu.y, cpu.stat, cpu.stp, cpu.cycl};
}
//: }}}

//: readbuf_vec {{{
// Vec copy utility function
pub fn readbuf_vec(to: &mut Vec<u8>, from: &mut Vec<u8>, start: &mut usize, size: usize) {
    for i in 0..size {
        to[i] = from[i + *start];
    }
    *start += size;
}
//: }}}
