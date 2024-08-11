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
    Sub { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },
    Andi { rd: usize, rs1: usize, imm: i32 },
    Or { rd: usize, rs1: usize, rs2: usize },
    Ori { rd: usize, rs1: usize, imm: i32 },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Xori { rd: usize, rs1: usize, imm: i32 },
    Sll { rd: usize, rs1: usize, rs2: usize },
    Slli { rd: usize, rs1: usize, shamt: u8 },
    Srl { rd: usize, rs1: usize, rs2: usize },
    Srli { rd: usize, rs1: usize, shamt: u8 },
    Sra { rd: usize, rs1: usize, rs2: usize },
    Srai { rd: usize, rs1: usize, shamt: u8 },
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
                let rd = rd as usize;
                let rs1 = rs1 as usize;
                let rs2 = rs2 as usize;

                match (funct3, funct7) {
                    (0b000, 0b0000000) => Self::Add { rd, rs1, rs2 },
                    (0b000, 0b0100000) => Self::Sub { rd, rs1, rs2 },
                    (0b111, 0b0000000) => Self::And { rd, rs1, rs2 },
                    (0b110, 0b0000000) => Self::Or { rd, rs1, rs2 },
                    (0b100, 0b0000000) => Self::Xor { rd, rs1, rs2 },
                    (0b001, 0b0000000) => Self::Sll { rd, rs1, rs2 },
                    (0b101, 0b0000000) => Self::Srl { rd, rs1, rs2 },
                    (0b101, 0b0100000) => Self::Sra { rd, rs1, rs2 },
                    _ => unimplemented!(),
                }
            }
            InstructionFormat::I {
                opcode: _,
                rd,
                funct3,
                rs1,
                imm0_11,
            } => {
                let rd = rd as usize;
                let rs1 = rs1 as usize;
                let mut imm = imm0_11 as i32;
                if imm & 0x800 != 0 {
                    imm |= 0xfffff000u32 as i32;
                }
                let shamt = (imm & 0x1f) as u8;

                match (funct3, imm0_11 >> 5) {
                    (0b000, _) => Self::Addi { rd, rs1, imm },
                    (0b111, _) => Self::Andi { rd, rs1, imm },
                    (0b110, _) => Self::Ori { rd, rs1, imm },
                    (0b100, _) => Self::Xori { rd, rs1, imm },
                    (0b001, _) => Self::Slli { rd, rs1, shamt },
                    (0b101, 0b0100000) => Self::Srai { rd, rs1, shamt },
                    (0b101, _) => Self::Srli { rd, rs1, shamt },
                    _ => unimplemented!(),
                }
            }
            _ => {
                unimplemented!()
            }
        };
        Ok(ins)
    }
}
