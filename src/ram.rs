pub struct Ram {
    pub memory: [u8; 0x800] // 2KB internal RAM
}


impl Ram {
    pub fn new() -> Self {
        Self {memory:[0; 0x800]}
    }

    pub fn setMemory(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}
