use crate::{cpu::Cpu, ram::Ram};

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
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
            print!("PC: 0x{:08x}: ", self.cpu.pc.load());
            let instruction = self.cpu.fetch(&self.ram)?;
            println!("instruction: 0x{:08x}", instruction);

            if self.cpu.pc.load() == 0 {
                break;
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
