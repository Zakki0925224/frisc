pub mod cpu;
pub mod emulator;
pub mod instruction;
pub mod ram;
pub mod register;

#[test]
fn test_add_addi() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x80, 0x10, 0x00, // ADDI x1, x1, 1
        0x13, 0x01, 0x21, 0x00, // ADDI x2, x2, 2
        0xb3, 0x81, 0x20, 0x00, // ADD x3, x1, x2
        0x13, 0x02, 0xf2, 0xff, // ADD x4, x4, -1
    ];
    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[1].load(), 1);
    assert_eq!(emulator.cpu.x_regs[2].load(), 2);
    assert_eq!(emulator.cpu.x_regs[3].load(), 3);
    assert_eq!(emulator.cpu.x_regs[4].load() as i32, -1);

    Ok(())
}

#[test]
fn test_sub() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x80, 0x20, 0x00, // ADDI x1, x1, 2
        0x13, 0x01, 0x31, 0x00, // ADDI x2, x2, 3
        0xb3, 0x01, 0x11, 0x40, // SUB  x3, x2, x1
        0x33, 0x82, 0x21, 0x40, // SUB  x4, x3, x2
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[3].load(), 1);
    assert_eq!(emulator.cpu.x_regs[4].load() as i32, -2);

    Ok(())
}
