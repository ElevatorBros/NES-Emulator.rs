// Vim folding 
// vim:foldmethod=marker

use crate::AddrM;
use crate::ADDRESSING_MODE_LOOKUP;
//use crate::Bus;
use crate::Cpu;


//: ASM_LOOKUP {{{
static ASM_LOOKUP : [&str; 0x100] = [
    " BRK"," ORA"," NUL"," NUL"," NUL"," ORA"," ASL"," NUL"," PHP"," ORA"," ASL","*AAC"," NUL"," ORA"," ASL"," NUL",
    " BPL"," ORA"," NUL"," NUL"," NUL"," ORA"," ASL"," NUL"," CLC"," ORA"," NUL"," NUL"," NUL"," ORA"," ASL"," NUL",
    " JSR"," AND"," NUL"," NUL"," BIT"," AND"," ROL"," NUL"," PLP"," AND"," ROL","*AAC"," BIT"," AND"," ROL"," NUL",
    " BMI"," AND"," NUL"," NUL"," NUL"," AND"," ROL"," NUL"," SEC"," AND"," NUL"," NUL"," NUL"," AND"," ROL"," NUL",
    " RTI"," EOR"," NUL"," NUL","*ASR"," EOR"," LSR"," NUL"," PHA"," EOR"," LSR"," NUL"," JMP"," EOR"," LSR"," NUL",
    " BVC"," EOR"," NUL"," NUL"," NUL"," EOR"," LSR"," NUL"," CLI"," EOR"," NUL"," NUL"," NUL"," EOR"," LSR"," NUL",
    " RTS"," ADC"," NUL"," NUL"," NUL"," ADC"," ROR"," NUL"," PLA"," ADC"," ROR","*ARR"," JMP"," ADC"," ROR"," NUL",
    " BVS"," ADC"," NUL"," NUL"," NUL"," ADC"," ROR"," NUL"," SEI"," ADC"," NUL"," NUL"," NUL"," ADC"," ROR"," NUL",
    " NUL"," STA"," NUL","*SAX"," STY"," STA"," STX","*SAX"," DEY"," NUL"," TXA"," NUL"," STY"," STA"," STX","*SAX",
    " BCC"," STA"," NUL"," NUL"," STY"," STA"," STX","*SAX"," TYA"," STA"," TXS"," NUL"," NUL"," STA"," NUL"," NUL",
    " LDY"," LDA"," LDX","*LAX"," LDY"," LDA"," LDX","*LAX"," TAY"," LDA"," TAX"," NUL"," LDY"," LDA"," LDX","*LAX",
    " BCS"," LDA"," NUL","*LAX"," LDY"," LDA"," LDX","*LAX"," CLV"," LDA"," TSX"," NUL"," LDY"," LDA"," LDX","*LAX",
    " CPY"," CMP"," NUL"," NUL"," CPY"," CMP"," DEC"," NUL"," INY"," CMP"," DEX","*AXS"," CPY"," CMP"," DEC"," NUL",
    " BNE"," CMP"," NUL"," NUL"," NUL"," CMP"," DEC"," NUL"," CLD"," CMP"," NUL"," NUL"," NUL"," CMP"," DEC"," NUL",
    " CPX"," SBC"," NUL"," NUL"," CPX"," SBC"," INC"," NUL"," INX"," SBC"," NOP"," NUL"," CPX"," SBC"," INC"," NUL",
    " BEQ"," SBC"," NUL"," NUL"," NUL"," SBC"," INC"," NUL"," SED"," SBC"," NUL"," NUL"," NUL"," SBC"," INC"," NUL",
];
//: }}}

//: print_asm {{{
pub fn get_asm(cpu: &Cpu) -> String {
    //let bus = cpu.bus;

    let mut asm_string: String;
    let opcode: u8 = cpu.bus.read(cpu.pc);
    //print!("{}", ASM_LOOKUP[opcode as usize]);
    //print!(" ");
    //asm_string += 
    asm_string = format!("{}", ASM_LOOKUP[opcode as usize]);
    match ADDRESSING_MODE_LOOKUP[opcode as usize] {
        AddrM::ACC => {
            //print!("A");
            asm_string = format!("{} A                         ", asm_string);
        }
        AddrM::ABS => {
            //print!("${:X}", cpu.bus.read_word_little(pc+1));
            let operand: u16 = cpu.bus.read_word_little(cpu.pc+1);
            asm_string = format!("{} ${:04X} = {:02X}                ", asm_string, operand, cpu.bus.read(operand));
        }
        AddrM::ADR => {
            //print!("${:X}", cpu.bus.read_word_little(pc+1));
            let operand: u16 = cpu.bus.read_word_little(cpu.pc+1);
            asm_string = format!("{} ${:04X}                     ", asm_string, operand);
        }
        AddrM::AIX => {
            //print!("${:X},X", cpu.bus.read_word_little(pc+1));
            let operand: u16 = cpu.bus.read_word_little(cpu.pc+1);
            let effective_address: u16 = operand + cpu.x as u16;
            asm_string = format!("{} ${:04X},X @ {:04X} = {:02X}       ", asm_string, operand, effective_address, cpu.bus.read(effective_address));
        }
        AddrM::AIY => {
            //print!("${:X},Y", cpu.bus.read_word_little(pc+1));
            let operand: u16 = cpu.bus.read_word_little(cpu.pc+1);
            let effective_address: u16 = operand.wrapping_add(cpu.y as u16);
            asm_string = format!("{} ${:04X},Y @ {:04X} = {:02X}       ", asm_string, operand, effective_address, cpu.bus.read(effective_address));
        }
        AddrM::IMD => {
            //print!("#${:X}", cpu.bus.read(pc+1));
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            asm_string = format!("{} #${:02X}                      ", asm_string, operand);
        }
        AddrM::IMP => {
            //print!("");
            asm_string = format!("{}                           ", asm_string);
        }
        AddrM::IND => {
            //print!("$({:X})", cpu.bus.read_word_little(pc+1));
            let operand: u16 = cpu.bus.read_word_little(cpu.pc+1);
            let effective_address: u16 = cpu.bus.read_word_little_wrap(operand);
            asm_string = format!("{} (${:04X}) = {:04X}            ", asm_string, operand, effective_address);
        } 
        AddrM::IIX => {
            //print!("$({:X},X)", cpu.bus.read(pc+1));
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            let mid_address: u8 = operand.wrapping_add(cpu.x);
            let low_byte: u8 = cpu.bus.read(mid_address as u16);
            let high_byte: u8 = cpu.bus.read((operand.wrapping_add(cpu.x)).wrapping_add(1) as u16);
            let effective_address: u16 = ((high_byte as u16) << 8) + low_byte as u16;

            asm_string = format!("{} (${:02X},X) @ {:02X} = {:04X} = {:02X}  ", asm_string, operand, mid_address, effective_address, cpu.bus.read(effective_address));
        }
        AddrM::IIY => {
            //print!("$({:X}),Y", cpu.bus.read(pc+1));
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            let low_byte: u8 = cpu.bus.read(operand as u16);
            let high_byte: u8 = cpu.bus.read(operand.wrapping_add(1) as u16);
            let raw_address: u16 = ((high_byte as u16) << 8) + low_byte as u16;

            let effective_address: u16 = raw_address.wrapping_add(cpu.y as u16);

            asm_string = format!("{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", asm_string, operand, raw_address, effective_address, cpu.bus.read(effective_address));
        }
        AddrM::REL => {
            //print!("${:X}", cpu.bus.read(pc+1));
            // Plus 2 because the the program counter will be incremented twice before the jump actually happens
            asm_string = format!("{} ${:04X}                     ", asm_string, ((cpu.pc as i32) + (cpu.bus.read(cpu.pc+1) as i32) + 2) as u16);
        }
        AddrM::ZPG => {
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            asm_string = format!("{} ${:02X} = {:02X}                  ", asm_string, operand, cpu.bus.read(operand as u16));
        }
        AddrM::ZIX => {
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            let effective_address: u8 = operand.wrapping_add(cpu.x);
            asm_string = format!("{} ${:02X},X @ {:02X} = {:02X}           ", asm_string, operand, effective_address, cpu.bus.read(effective_address as u16));
        }
        AddrM::ZIY => {
            let operand: u8 = cpu.bus.read(cpu.pc+1);
            let effective_address: u8 = operand.wrapping_add(cpu.y);
            asm_string = format!("{} ${:02X},Y @ {:02X} = {:02X}           ", asm_string, operand, effective_address, cpu.bus.read(effective_address as u16));
        }
        AddrM::NUL => {
            asm_string = format!("Invalid Opcode                ");
        }
    }
    //print!("\n");
    //print!(" ");
    //format!("{}|", asm_string)
    asm_string
}
//: }}}

pub fn readbuf(to: &Vec<u8>, from: &Vec<u8>, start: &mut usize, size: usize) {
    for i in *start..(*start + size) {
        to[i] = from[i];
    }
    *start += size;
}
