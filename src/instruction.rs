#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
}

impl Instruction {
    pub fn parse(instruction: u32) -> anyhow::Result<Self> {
        /* 00..06 bits */
        let opcode = instruction & 0x7f;
        /* 07..11 bits */
        let rd = (instruction >> 7) & 0x1f;
        /* 12..14 bits */
        let funct3 = (instruction >> 12) & 0x7;
        /* 15..19 bits */
        let rs1 = (instruction >> 15) & 0x1f;
        /* 20..24 bits */
        let rs2 = (instruction >> 20) & 0x1f;
        /* 25..31 bits */
        let funct7 = (instruction >> 25) & 0x7f;

        if opcode == 0b0110011 && funct3 == 0b000 && funct7 == 0b0000000 {
            Ok(Self::Add {
                rd: rd as usize,
                rs1: rs1 as usize,
                rs2: rs2 as usize,
            })
        } else {
            Err(anyhow::anyhow!("Invalid instruction"))
        }
    }
}
