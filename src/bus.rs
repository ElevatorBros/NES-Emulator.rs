struct Bus {
    ram: &Ram, // 2KB Internal RAM
}


impl Bus {
    // Setup Functions
    fn setup(ram: &Ram) {}

    // Interface Functions
    fn read(addr: u16) -> u8 {}
    fn write(addr: u16, value: u8) {}
}
