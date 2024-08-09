use crate::{
    ram::Ram,
    register::{ProgramCounter, Register},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuState {
    Reset,
    Fetch,
    Decode,
    Execute,
}

#[derive(Debug)]
pub struct Cpu {
    pub x_regs: [Register; 32],
    pub pc: ProgramCounter,
    pub state: CpuState,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x_regs: [Register::default(); 32],
            pc: ProgramCounter::default(),
            state: CpuState::Reset,
        }
    }
}

impl Cpu {
    pub fn reset(&mut self) {
        self.x_regs = [Register::default(); 32];
        self.pc = ProgramCounter::default();
        self.state = CpuState::Reset;
    }

    pub fn fetch(&mut self, ram: &Ram) -> anyhow::Result<u32> {
        self.state = CpuState::Fetch;

        let pc = self.pc.load();
        if pc as usize >= ram.size() {
            return Err(anyhow::anyhow!("PC is out of bounds memory"));
        }

        let instruction = ram.read32(pc);
        self.pc.increment();

        Ok(instruction)
    }
}
