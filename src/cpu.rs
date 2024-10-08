use serde::Serialize;

use crate::{
    instruction::{Instruction, InstructionFormat},
    mmio_device::MmioDeviceInterface,
    ram::Ram,
    register::{ProgramCounter, Register},
    step_log,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
    pub step: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x_regs: [Register::default(); 32],
            pc: ProgramCounter::default(),
            state: CpuState::Reset,
            step: 0,
        }
    }
}

impl Cpu {
    pub fn reset(&mut self) {
        self.x_regs = [Register::default(); 32];
        self.pc = ProgramCounter::default();
        self.state = CpuState::Reset;
        self.step = 0;
    }

    pub fn fetch_decode_execute(
        &mut self,
        ram: &mut Ram,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
        print_instruction_log: bool,
    ) -> anyhow::Result<step_log::CpuStep> {
        let fetched_instruction = self.fetch(ram)?;
        let decoded_instruction = self.decode(fetched_instruction)?;
        let ram_writes = self.execute(decoded_instruction, ram, mmio_devices)?;

        let cpu_step = step_log::CpuStep {
            step: self.step,
            fetched_instruction,
            decoded_instruction,
            cpu_state: step_log::CpuStateLog::new(&self),
            ram_writes,
        };
        self.step += 1;

        if print_instruction_log {
            let pc = cpu_step.cpu_state.pc;
            println!("0x{:08x} 0x{:08x} {:?}", pc, cpu_step.fetched_instruction, cpu_step.decoded_instruction);
        }

        Ok(cpu_step)
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

    fn execute(
        &mut self,
        instruction: Instruction,
        ram: &mut Ram,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) -> anyhow::Result<Vec<step_log::RamWrite>> {
        match self.state {
            CpuState::Decode => (),
            _ => return Err(anyhow::anyhow!("Invalid state for execute")),
        }

        self.state = CpuState::Execute;
        let mut ram_write_logs = Vec::new();

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
                let mut value = ram.load8_with_mmio(addr, mmio_devices) as u32;
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
                let value = ram.load8_with_mmio(addr, mmio_devices) as u32;
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
                ram.store8_with_mmio(addr, value, mmio_devices);
                self.pc.increment();

                ram_write_logs.push(step_log::RamWrite::new(addr, value))
            }
            Instruction::Lh { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let mut value = ram.load16_with_mmio(addr, mmio_devices) as u32;
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
                let value = ram.load16_with_mmio(addr, mmio_devices) as u32;
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
                ram.store16_with_mmio(addr, value, mmio_devices);
                self.pc.increment();

                let values = value.to_le_bytes();
                ram_write_logs.push(step_log::RamWrite::new(addr, values[0]));
                ram_write_logs.push(step_log::RamWrite::new(addr + 1, values[1]));
            }
            Instruction::Lw { rd, rs1, offset } => {
                let mut addr = self.load_x_regs(rs1)?;
                addr = if offset >= 0 {
                    addr + offset as u32
                } else {
                    addr - (-offset) as u32
                };
                let value = ram.load32_with_mmio(addr, mmio_devices);
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
                ram.store32_with_mmio(addr, value, mmio_devices);
                self.pc.increment();

                let values = value.to_le_bytes();
                ram_write_logs.push(step_log::RamWrite::new(addr, values[0]));
                ram_write_logs.push(step_log::RamWrite::new(addr + 1, values[1]));
                ram_write_logs.push(step_log::RamWrite::new(addr + 2, values[2]));
                ram_write_logs.push(step_log::RamWrite::new(addr + 3, values[3]));
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
            Instruction::Jalr { rd, rs1, offset } => {
                let t = self.pc.load() + 4;
                let mut pc = self.load_x_regs(rs1)?;
                pc = if offset >= 0 {
                    pc + offset as u32
                } else {
                    pc - (-offset) as u32
                };
                pc &= !1;
                self.pc.store(pc);
                self.store_x_regs(rd, t)?;
            }
            Instruction::Beq { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                let mut pc = self.pc.load();
                pc = if x_rs1 == x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Bne { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                let mut pc = self.pc.load();
                pc = if x_rs1 != x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Blt { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = self.load_x_regs(rs2)? as i32;
                let mut pc = self.pc.load();
                pc = if x_rs1 < x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Bge { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)? as i32;
                let x_rs2 = self.load_x_regs(rs2)? as i32;
                let mut pc = self.pc.load();
                pc = if x_rs1 >= x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Bltu { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                let mut pc = self.pc.load();
                pc = if x_rs1 < x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Bgeu { rs1, rs2, offset } => {
                let x_rs1 = self.load_x_regs(rs1)?;
                let x_rs2 = self.load_x_regs(rs2)?;
                let mut pc = self.pc.load();
                pc = if x_rs1 >= x_rs2 {
                    if offset >= 0 {
                        pc + offset as u32
                    } else {
                        pc - (-offset) as u32
                    }
                } else {
                    pc + 4
                };
                self.pc.store(pc);
            }
            Instruction::Lui { rd, imm } => {
                self.store_x_regs(rd, imm)?;
                self.pc.increment();
            }
            Instruction::Auipc { rd, imm } => {
                let pc = self.pc.load() + imm;
                self.store_x_regs(rd, pc)?;
                self.pc.increment();
            }
            Instruction::Fence { pred, succ } => {
                return Err(anyhow::anyhow!(
                    "Fence (pred: 0x{:x}, succ: 0x{:x})",
                    pred,
                    succ
                ));
            }
            Instruction::Ecall => {
                return Err(anyhow::anyhow!("Ecall"));
            }
            Instruction::Ebreak => {
                return Err(anyhow::anyhow!("Ebreak"));
            }
        }

        Ok(ram_write_logs)
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
