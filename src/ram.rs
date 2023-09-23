#![allow(dead_code)]
#![allow(unused_variables)]
pub struct Ram {
    pub cpu_memory: [u8; 0x800], // 2KB internal RAM
    pub ppu_memory: [u8; 0x4000],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            cpu_memory: [0; 0x800],
            ppu_memory: [0; 0x4000],
        }
    }

    pub fn get_cpu_memory(&mut self, addr: u16) -> u8 {
        self.cpu_memory[addr as usize]
    }

    pub fn set_cpu_memory(&mut self, addr: u16, value: u8) {
        self.cpu_memory[addr as usize] = value;
    }

    pub fn get_ppu_memory(&mut self, addr: u16) -> u8 {
        println!(
            "Read {:#x} from {:#x}",
            addr, self.ppu_memory[addr as usize]
        );
        self.ppu_memory[addr as usize]
    }

    pub fn set_ppu_memory(&mut self, addr: u16, value: u8) {
        println!("Wrote {:#x} to {:#x}", value, addr);
        self.ppu_memory[addr as usize] = value;
    }
}
