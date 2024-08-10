pub mod cpu;
pub mod emulator;
pub mod instruction;
pub mod ram;
pub mod register;

#[test]
fn test_main() {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x80, 0x10, 0x00, // addi x1, x1, 1
        0x13, 0x01, 0x21, 0x00, // addi x2, x2, 2
        0xb3, 0x81, 0x20, 0x00, // add x3, x1, x2
    ];
    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run().unwrap();

    assert_eq!(emulator.cpu.x_regs[1].load(), 1);
    assert_eq!(emulator.cpu.x_regs[2].load(), 2);
    assert_eq!(emulator.cpu.x_regs[3].load(), 3);
}
