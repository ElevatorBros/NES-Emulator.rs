struct Bus {
    ram: &Ram, // 2KB Internal RAM
    cart: &Cart, 
}


impl Bus {
    // Setup Functions
    pub fn new(ram: &Ram, cart: &Cart) -> Self {
        Self { ram, cart}
    }

    // Interface Functions
    pub fn read(&self, addr: u16) -> u8 {
        if (addr < 0x2000) { // Internal RAM
            addr = addr % 0x800;
            return self.ram.memory[addr];
        } else { // Cartridge space 
            if (addr >= 0x8000) {
                addr -= 0x8000;
                return self.cart.ROM[addr];
            }
        }
    }

    pub fn write(&self, addr: u16, value: u8) {
        if (addr < 0x2000) { // Internal RAM
            addr = addr % 0x800;
            self.ram.memory[addr] = value;
        } else { // Cartridge space 
            if (addr >= 0x8000) {
                addr -= 0x8000;
                self.cart.ROM[addr] = value;
            }
        }
    }
}
