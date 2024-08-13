pub mod cpu;
pub mod emulator;
pub mod instruction;
pub mod mmio_device;
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

    assert_eq!(emulator.cpu.x_regs[8].load(), 839);
    assert_eq!(emulator.cpu.x_regs[9].load() as i32, -1);

    Ok(())
}

#[test]
fn test_xor_xori() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x03, 0x70, 0x33, // ADDI x7, x0, 823
        0x13, 0x04, 0xe0, 0x55, // ADDI x8, x0, 1374
        0xb3, 0xc4, 0x83, 0x00, // XOR x9, x7, x8
        0x13, 0xc5, 0xd4, 0xe7, // XORI x10, x9, -387
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[9].load(), 1641);
    assert_eq!(emulator.cpu.x_regs[10].load() as i32, -2028);

    Ok(())
}

#[test]
fn test_sll_slli_srl_srli() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x00, 0x70, 0x01, // ADDI x1, x0, 23
        0x13, 0x01, 0x50, 0x00, // ADDI x2, x0, 5
        0x33, 0x92, 0x20, 0x00, // SLL x4, x1, x2
        0x93, 0x12, 0x31, 0x00, // SLLI x5, x2, 3
        0x33, 0x53, 0x52, 0x00, // SRL x6, x4, x5
        0x93, 0xd3, 0x30, 0x00, // SRLI x7, x1, 3
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[4].load(), 736);
    assert_eq!(emulator.cpu.x_regs[5].load(), 40);
    assert_eq!(emulator.cpu.x_regs[6].load(), 2);
    assert_eq!(emulator.cpu.x_regs[7].load(), 2);

    Ok(())
}

#[test]
fn test_sra_srai() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x00, 0xf0, 0xff, // ADDI x1, x0, -1
        0x13, 0x01, 0x40, 0x00, // ADDI x2, x0, 4
        0xb3, 0xd1, 0x20, 0x40, // SRA x3, x1, x2
        0x13, 0xd2, 0xb0, 0x40, // SRAI x4, x1, 11
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[3].load() as i32, -1);
    assert_eq!(emulator.cpu.x_regs[4].load() as i32, -1);

    Ok(())
}

#[test]
fn test_slt_slti_sltu_sltiu() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x00, 0xb0, 0x07, // ADDI x1, x0, 123
        0x13, 0x01, 0xf0, 0xff, // ADDI x2, x0, -1
        0xb3, 0xa1, 0x00, 0x00, // SLT x3, x1, x0
        0x13, 0xa2, 0xb0, 0x07, // SLTI x4, x1, 123
        0xb3, 0xb2, 0x20, 0x00, // SLTU x5, x1, x2
        0x13, 0x33, 0x01, 0x00, // SLTIU x6, x2, 0
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[3].load(), 0);
    assert_eq!(emulator.cpu.x_regs[4].load(), 0);
    assert_eq!(emulator.cpu.x_regs[5].load(), 1);
    assert_eq!(emulator.cpu.x_regs[6].load(), 0);

    Ok(())
}

#[test]
fn test_lb_lbu_sb() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x00, 0x70, 0xfe, // ADDI x1, x0, -25
        0x23, 0x00, 0x10, 0x00, // SB x1, 0(x0)
        0x03, 0x01, 0x00, 0x00, // LB x2, 0(x0)
        0x83, 0x41, 0x00, 0x00, // LBU x3, 0(x0)
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[2].load() as i32, -25);
    assert_eq!(emulator.cpu.x_regs[3].load(), 231);
    assert_eq!(emulator.ram.load8(0), 231);

    Ok(())
}

#[test]
fn test_lw_sw() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0x93, 0x00, 0x70, 0xfe, // ADDI x1, x0, -25
        0x23, 0x20, 0x10, 0x00, // SW x1, 0(x0)
        0x03, 0x21, 0x00, 0x00, // LW x2, 0(x0)
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[2].load() as i32, -25);
    assert_eq!(emulator.ram.load32(0) as i32, -25);

    Ok(())
}

#[test]
fn test_jal() -> anyhow::Result<()> {
    use emulator::Emulator;

    let ram_data = vec![
        0xef, 0x00, 0x00, 0x08, // JAL x1, 128
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run()?;

    assert_eq!(emulator.cpu.x_regs[1].load(), 4);
    assert_eq!(emulator.cpu.pc.load(), 128);

    Ok(())
}

#[test]
fn test_hello() -> anyhow::Result<()> {
    use emulator::Emulator;

    // hello.c
    // void _start()
    // {
    //     volatile char *addr = (volatile char *)0x123;
    //     *addr = 'A';
    // }
    // clang --target=riscv32 -march=rv32i -mabi=ilp32 -nostdlib -o hello.elf hello.c

    let hello_elf = &[
        0x13, 0x01, 0x01, 0xff, // ADDI sp, sp, -16
        0x23, 0x26, 0x11, 0x00, // SW ra, 12(sp)
        0x23, 0x24, 0x81, 0x00, // SW s0, 8(sp)
        0x13, 0x04, 0x01, 0x01, // ADDI s0, sp, 16
        0x13, 0x05, 0x30, 0x12, // LI a0, 291
        0x23, 0x2a, 0xa4, 0xfe, // SW a0, -12(s0)
        0x83, 0x25, 0x44, 0xff, // LW a1, -12(s0)
        0x13, 0x05, 0x10, 0x04, // LI a0, 65
        0x23, 0x80, 0xa5, 0x00, // SB a0, 0(a1)
        0x83, 0x20, 0xc1, 0x00, // LW ra, 12(sp)
        0x03, 0x24, 0x81, 0x00, // LW s0, 8(sp)
        0x13, 0x01, 0x01, 0x01, // ADDI sp, sp, 16
        //0x67, 0x80, 0x00, 0x00, // ret
        0x73, 0x00, 0x10, 0x00, // EBREAK
    ];

    let mut ram = vec![0u8; 0x200000];
    ram[..hello_elf.len()].copy_from_slice(hello_elf);

    let mut emulator = Emulator::new(ram);
    emulator.reset();
    emulator.cpu.x_regs[2].store(0x1000); // sp
    let _ = emulator.run();

    assert_eq!(emulator.ram.load8(0x123), 65); // A

    Ok(())
}

#[test]
fn test_debug_exit() -> anyhow::Result<()> {
    use emulator::Emulator;
    use mmio_device::debug_exit::DebugExit;

    let ram_data = vec![
        0x93, 0x00, 0xe0, 0x0a, // ADDI x1, x0, 0xae
        0x13, 0x01, 0x40, 0x0f, // ADDI x2, x0, 0xf4
        0x23, 0x00, 0x11, 0x00, // SB x1, 0(x2)
        0x00, 0x00, 0x00, 0x00, // unreachable
    ];

    let mut emulator = Emulator::new(ram_data);
    emulator.register_mmio_device(Box::new(DebugExit::default()));
    emulator.reset();
    let exit_code = emulator.run()?;

    assert_eq!(exit_code, 0xae);

    Ok(())
}
