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

#[test]
fn test_and_andi() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x81, 0xa0, 0x0e, // ADDI x3, x1, 234
        0x13, 0x02, 0x21, 0x09, // ADDI x4, x2, 146
        0x33, 0x75, 0x32, 0x00, // AND x10, x4, x3
        0x13, 0x76, 0x55, 0xff, // ANDI x12, x10, -11
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[3].load(), 234);
    assert_eq!(emulator.cpu.x_regs[4].load(), 146);
    assert_eq!(emulator.cpu.x_regs[10].load(), 130);
    assert_eq!(emulator.cpu.x_regs[12].load(), 128);

    Ok(())
}

#[test]
fn test_or_ori() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x82, 0x50, 0x34, // ADDI x5, x1, 837
        0x13, 0x03, 0x71, 0x10, // ADDI x6, x2, 263
        0x33, 0xe4, 0x62, 0x00, // OR x8, x5, x6
        0x93, 0x64, 0xf4, 0xff, // ORI x9, x8, -1
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[5].load(), 837);
    assert_eq!(emulator.cpu.x_regs[6].load(), 263);
    assert_eq!(emulator.cpu.x_regs[8].load(), 839);
    assert_eq!(emulator.cpu.x_regs[9].load() as i32, -1);

    Ok(())
}
