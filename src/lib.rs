pub mod cpu;
pub mod emulator;
pub mod ram;
pub mod register;

#[test]
fn test_main() {
    use emulator::Emulator;

    let ram_data = vec![0x01, 0x02, 0x03, 0x04];
    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run().unwrap()
}
