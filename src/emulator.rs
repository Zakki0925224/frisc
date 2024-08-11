use crate::{cpu::Cpu, ram::Ram};

#[derive(Debug, Default)]
pub struct Emulator {
    pub cpu: Cpu,
    pub ram: Ram,
}

impl Emulator {
    pub fn new(ram_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::default(),
            ram: Ram::new_with_data(ram_data),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.cpu.fetch_decode_execute(&mut self.ram)?;
            if self.cpu.pc.load() as usize >= self.ram.size() {
                break;
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
