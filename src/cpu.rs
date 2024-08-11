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

    pub fn fetch_decode_execute(&mut self, ram: &mut Ram) -> anyhow::Result<()> {
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
        println!("{:?}", ram);
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

        let instruction = ram.load32(pc);
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

    fn execute(&mut self, ram: &mut Ram, instruction: Instruction) -> anyhow::Result<()> {
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
                self.pc.increment();
            }
            Instruction::Addi { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 + imm as i32) as u32)?;
                self.pc.increment();
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = self.load_x_regs(rs2)? as i32;
                self.store_x_regs(rd, (x_rs1 - x_rs2) as u32)?;
                self.pc.increment();
            }
            Instruction::And { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 & x_rs2)?;
                self.pc.increment();
            }
            Instruction::Andi { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 & imm as i32) as u32)?;
                self.pc.increment();
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 | x_rs2)?;
                self.pc.increment();
            }
            Instruction::Ori { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 | imm as i32) as u32)?;
                self.pc.increment();
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, x_rs1 ^ x_rs2)?;
                self.pc.increment();
            }
            Instruction::Xori { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 ^ imm as i32) as u32)?;
                self.pc.increment();
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = (self.load_x_regs(rs2)? & 0x1f) as u8;
                self.store_x_regs(rd, x_rs1 << x_rs2)?;
                self.pc.increment();
            }
            Instruction::Slli { rd, rs1, shamt } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                self.store_x_regs(rd, x_rs1 << shamt)?;
                self.pc.increment();
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = (self.load_x_regs(rs2)? & 0x1f) as u8;
                self.store_x_regs(rd, x_rs1 >> x_rs2)?;
                self.pc.increment();
            }
            Instruction::Srli { rd, rs1, shamt } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                self.store_x_regs(rd, x_rs1 >> shamt)?;
                self.pc.increment();
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = (self.load_x_regs(rs2)? & 0x1f) as u8;
                self.store_x_regs(rd, (x_rs1 >> x_rs2) as u32)?;
                self.pc.increment();
            }
            Instruction::Srai { rd, rs1, shamt } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 >> shamt) as u32)?;
                self.pc.increment();
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = self.load_x_regs(rs2)? as i32;
                self.store_x_regs(rd, (x_rs1 < x_rs2) as u32)?;
                self.pc.increment();
            }
            Instruction::Slti { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                self.store_x_regs(rd, (x_rs1 < imm as i32) as u32)?;
                self.pc.increment();
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                self.store_x_regs(rd, (x_rs1 < x_rs2) as u32)?;
                self.pc.increment();
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                self.store_x_regs(rd, (x_rs1 < imm as u32) as u32)?;
                self.pc.increment();
            }
            Instruction::Lb { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let mut value = ram.load8(addr) as u32;
                if value & 0x80 != 0 {
                    value |= 0xffffff00;
                }
                self.store_x_regs(rd, value)?;
                self.pc.increment();
            }
            Instruction::Lbu { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = ram.load8(addr) as u32;
                self.store_x_regs(rd, value)?;
                self.pc.increment();
            }
            Instruction::Sb { rs1, rs2, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = self.load_x_regs(rs2)? as u8;
                ram.store8(addr, value);
                self.pc.increment();
            }
            Instruction::Lh { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let mut value = ram.load16(addr) as u32;
                if value & 0x8000 != 0 {
                    value |= 0xffff0000;
                }
                self.store_x_regs(rd, value)?;
                self.pc.increment();
            }
            Instruction::Lhu { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = ram.load16(addr) as u32;
                self.store_x_regs(rd, value)?;
                self.pc.increment();
            }
            Instruction::Sh { rs1, rs2, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = self.load_x_regs(rs2)? as u16;
                ram.store16(addr, value);
                self.pc.increment();
            }
            Instruction::Lw { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = ram.load32(addr);
                self.store_x_regs(rd, value)?;
                self.pc.increment();
            }
            Instruction::Sw { rs1, rs2, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = self.load_x_regs(rs2)?;
                ram.store32(addr, value);
                self.pc.increment();
            }
            Instruction::Jal { rd, offset } => {
                let mut pc = self.pc.load();
                self.store_x_regs(rd, pc + 4)?;
                pc = if offset >= 0 {
                    pc + offset as u32
                } else {
                    pc - (-offset) as u32
                };
                self.pc.store(pc);
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
