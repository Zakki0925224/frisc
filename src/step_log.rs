use serde::Serialize;

use crate::{
    cpu::{Cpu, CpuState},
    instruction::Instruction,
    mmio_device::RequestFromDevice,
};

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CpuStateLog {
    pub zero: u32,
    pub ra: u32,
    pub sp: u32,
    pub gp: u32,
    pub tp: u32,
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub s0: u32,
    pub s1: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub pc: u32,
    pub state: CpuState,
}

impl CpuStateLog {
    pub fn new(cpu: &Cpu) -> Self {
        let x_regs: Vec<u32> = cpu.x_regs.to_vec().iter().map(|r| r.load()).collect();

        Self {
            zero: x_regs[0],
            ra: x_regs[1],
            sp: x_regs[2],
            gp: x_regs[3],
            tp: x_regs[4],
            t0: x_regs[5],
            t1: x_regs[6],
            t2: x_regs[7],
            s0: x_regs[8],
            s1: x_regs[9],
            a0: x_regs[10],
            a1: x_regs[11],
            a2: x_regs[12],
            a3: x_regs[13],
            a4: x_regs[14],
            a5: x_regs[15],
            a6: x_regs[16],
            a7: x_regs[17],
            s2: x_regs[18],
            s3: x_regs[19],
            s4: x_regs[20],
            s5: x_regs[21],
            s6: x_regs[22],
            s7: x_regs[23],
            s8: x_regs[24],
            s9: x_regs[25],
            s10: x_regs[26],
            s11: x_regs[27],
            t3: x_regs[28],
            t4: x_regs[29],
            t5: x_regs[30],
            t6: x_regs[31],
            pc: cpu.pc.load(),
            state: cpu.state,
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct RamWrite {
    pub addr: u32,
    pub value: u8,
}

impl RamWrite {
    pub fn new(addr: u32, value: u8) -> Self {
        Self { addr, value }
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct CpuStep {
    pub step: usize,
    pub fetched_instruction: u32,
    pub decoded_instruction: Instruction,
    pub cpu_state: CpuStateLog,
    pub ram_writes: Vec<RamWrite>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct DeviceRequest {
    pub step: usize,
    pub req: RequestFromDevice,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct Log {
    pub init_cpu_state: CpuStateLog,
    pub init_ram: Vec<u8>,
    pub steps: Vec<CpuStep>,
    pub dev_reqs: Vec<DeviceRequest>,
}
