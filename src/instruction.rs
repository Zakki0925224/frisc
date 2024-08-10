pub enum InstructionFormat {
    R {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    I {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm0_11: u16,
    },
    S {
        opcode: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm0_11: u16,
    },
    U {
        opcode: u8,
        rd: u8,
        imm12_31: u32,
    },
}

impl Into<u32> for InstructionFormat {
    fn into(self) -> u32 {
        match self {
            InstructionFormat::R {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                (funct7 as u32) << 25
                    | (rs2 as u32) << 20
                    | (rs1 as u32) << 15
                    | (funct3 as u32) << 12
                    | (rd as u32) << 7
                    | opcode as u32
            }
            InstructionFormat::I {
                opcode,
                rd,
                funct3,
                rs1,
                imm0_11,
            } => {
                (imm0_11 as u32) << 20
                    | (rs1 as u32) << 15
                    | (funct3 as u32) << 12
                    | (rd as u32) << 7
                    | opcode as u32
            }
            InstructionFormat::S {
                opcode,
                funct3,
                rs1,
                rs2,
                imm0_11,
            } => {
                ((imm0_11 as u32) >> 5) << 25
                    | (rs2 as u32) << 20
                    | (rs1 as u32) << 15
                    | (funct3 as u32) << 12
                    | ((imm0_11 & 0x1f) as u32) << 7
                    | opcode as u32
            }
            InstructionFormat::U {
                opcode,
                rd,
                imm12_31,
            } => imm12_31 << 12 | (rd as u32) << 7 | opcode as u32,
        }
    }
}

impl InstructionFormat {
    pub fn parse(instruction: u32) -> anyhow::Result<Self> {
        let opcode = (instruction & 0x7f) as u8;
        let rd = ((instruction >> 7) & 0x1f) as u8;
        let funct3 = ((instruction >> 12) & 0x7) as u8;
        let rs1 = ((instruction >> 15) & 0x1f) as u8;

        let format = match opcode {
            0b0110011 => {
                let rs2 = ((instruction >> 20) & 0x1f) as u8;
                let funct7 = ((instruction >> 25) & 0x7f) as u8;

                Self::R {
                    opcode,
                    rd,
                    funct3,
                    rs1,
                    rs2,
                    funct7,
                }
            }
            0b0010011 => {
                let imm0_11 = ((instruction >> 20) & 0xfff) as u16;

                Self::I {
                    opcode,
                    rd,
                    funct3,
                    rs1,
                    imm0_11,
                }
            }
            _ => return Err(anyhow::anyhow!("Invalid instruction")),
        };
        Ok(format)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Addi { rd: usize, rs1: usize, imm: i32 },
}

impl Instruction {
    pub fn parse(instruction_format: InstructionFormat) -> anyhow::Result<Self> {
        let ins = match instruction_format {
            InstructionFormat::R {
                opcode: _,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => {
                if funct3 == 0b000 && funct7 == 0b0000000 {
                    Self::Add {
                        rd: rd as usize,
                        rs1: rs1 as usize,
                        rs2: rs2 as usize,
                    }
                } else {
                    unimplemented!()
                }
            }
            InstructionFormat::I {
                opcode: _,
                rd,
                funct3,
                rs1,
                imm0_11,
            } => {
                if funct3 == 0b000 {
                    Self::Addi {
                        rd: rd as usize,
                        rs1: rs1 as usize,
                        imm: imm0_11 as i32, // TODO
                    }
                } else {
                    unimplemented!()
                }
            }
            _ => {
                unimplemented!()
            }
        };
        Ok(ins)
    }
}
