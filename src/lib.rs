pub mod cpu;
pub mod emulator;
pub mod instruction;
pub mod ram;
pub mod register;

#[test]
fn test_main() {
    use emulator::Emulator;

    let ram_data = vec![0x33, 0x00, 0x00, 0x00]; // add 0, 0, 0
    let mut emulator = Emulator::new(ram_data);
    emulator.reset();
    emulator.run().unwrap()
}
