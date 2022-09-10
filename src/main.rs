fn main() {
    let mainRam = Ram::new();
    let mainCart = Cart::new();
    mainCart.ROM[0x00] = 0xA9;
    mainCart.ROM[0x01] = 0x07;

    let mainBus = Bus::new(&mainRam, &mainCart);

    let mainCpu = Cpu::new(&mainBus);

    for i in 0..10 {
        mainCpu.clock();
    }
}
