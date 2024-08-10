use crate::{
    instruction::Instruction,
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

    pub fn fetch_decode_execute(&mut self, ram: &Ram) -> anyhow::Result<()> {
        let pc = self.pc.load();
        println!("PC: 0x{:08x}", pc);

        print!("fetching ...");
        let instruction = self.fetch(ram)?;
        println!("0x{:08x}", instruction);

        print!("decoding ...");
        let decoded_instruction = self.decode(ram, instruction)?;
        println!("{:?}", decoded_instruction);

        print!("executing...");
        self.execute(ram, decoded_instruction)?;
        println!("done!");
        println!("{:?}", self);

        self.pc.increment();
        Ok(())
    }

    fn fetch(&mut self, ram: &Ram) -> anyhow::Result<u32> {
        match self.state {
            CpuState::Reset | CpuState::Execute => (),
            _ => return Err(anyhow::anyhow!("Invalid state for fetch")),
        }

        self.state = CpuState::Fetch;

        let pc = self.pc.load();
        if pc as usize >= ram.size() {
            return Err(anyhow::anyhow!("PC is out of bounds memory"));
        }

        let instruction = ram.read32(pc);
        Ok(instruction)
    }

    fn decode(&mut self, ram: &Ram, instruction: u32) -> anyhow::Result<Instruction> {
        match self.state {
            CpuState::Fetch => (),
            _ => return Err(anyhow::anyhow!("Invalid state for decode")),
        }

        self.state = CpuState::Decode;
        let parsed_instruction = Instruction::parse(instruction)?;

        Ok(parsed_instruction)
    }

    fn execute(&mut self, ram: &Ram, instruction: Instruction) -> anyhow::Result<()> {
        match self.state {
            CpuState::Decode => (),
            _ => return Err(anyhow::anyhow!("Invalid state for execute")),
        }

        self.state = CpuState::Execute;

        match instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                self.x_regs[rd].store(self.x_regs[rs1].load() + self.x_regs[rs2].load());
            }
        }

        Ok(())
    }
}
