// Basicly mapper 0 right now

pub struct Cart {
    pub ROM: [u8; 0x8000] // 32 KB of ROM
}

impl Cart {
    pub fn new() -> Self {
        Self {ROM:[0; 0x8000]}
    }
}
