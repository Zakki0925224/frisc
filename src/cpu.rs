use crate::{
    instruction::{Instruction, InstructionFormat},
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
        print!("fetching ...");
        let instruction = self.fetch(ram)?;
        println!("0x{:08x}", instruction);

        print!("decoding ...");
        let decoded_instruction = self.decode(instruction)?;
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

    fn decode(&mut self, instruction: u32) -> anyhow::Result<Instruction> {
        match self.state {
            CpuState::Fetch => (),
            _ => return Err(anyhow::anyhow!("Invalid state for decode")),
        }

        self.state = CpuState::Decode;
        let instruction_format = InstructionFormat::parse(instruction)?;
        let parsed_instruction = Instruction::parse(instruction_format)?;
        Ok(parsed_instruction)
    }

    fn execute(&mut self, _ram: &Ram, instruction: Instruction) -> anyhow::Result<()> {
        match self.state {
            CpuState::Decode => (),
            _ => return Err(anyhow::anyhow!("Invalid state for execute")),
        }

        self.state = CpuState::Execute;

        match instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 + x_rs2)?;
            }
            Instruction::Addi { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 + imm) as u32)?;
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = self.load_x_regs(rs2)? as i32;
                self.store_x_regs(rd, (x_rs1 - x_rs2) as u32)?;
            }
            Instruction::And { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 & x_rs2)?;
            }
            Instruction::Andi { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 & imm) as u32)?;
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 | x_rs2)?;
            }
            Instruction::Ori { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 | imm) as u32)?;
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 ^ x_rs2)?;
            }
            Instruction::Xori { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 ^ imm) as u32)?;
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = (self.load_x_regs(rs2)? & 0x1f) as u8;
                self.store_x_regs(rd, x_rs1 << x_rs2)?;
            }
            Instruction::Slli { rd, rs1, shamt } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                self.store_x_regs(rd, x_rs1 << shamt)?;
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = (self.load_x_regs(rs2)? & 0x1f) as u8;
                self.store_x_regs(rd, x_rs1 >> x_rs2)?;
            }
            Instruction::Srli { rd, rs1, shamt } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                self.store_x_regs(rd, x_rs1 >> shamt)?;
            }
        }

        Ok(())
    }

    fn load_x_regs(&mut self, index: usize) -> anyhow::Result<u32> {
        if index >= self.x_regs.len() {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }

        Ok(self.x_regs[index].load())
    }

    fn store_x_regs(&mut self, index: usize, value: u32) -> anyhow::Result<()> {
        if index >= self.x_regs.len() {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }

        if index != 0 {
            self.x_regs[index].store(value);
        }

        Ok(())
    }
}
