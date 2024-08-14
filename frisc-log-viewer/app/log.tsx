export interface StepLog
{
    init_cpu_state: CpuState;
    init_ram: number[];
    steps: CpuStep[];
    dev_reqs: DeviceRequest[];
}

export interface DeviceRequest
{
    step: number;
    req: RequestFromDevice;
}

export interface RequestFromDevice
{
    Exit: number;
}

export interface CpuState
{
    zero: number;
    ra: number;
    sp: number;
    gp: number;
    tp: number;
    t0: number;
    t1: number;
    t2: number;
    s0: number;
    s1: number;
    a0: number;
    a1: number;
    a2: number;
    a3: number;
    a4: number;
    a5: number;
    a6: number;
    a7: number;
    s2: number;
    s3: number;
    s4: number;
    s5: number;
    s6: number;
    s7: number;
    s8: number;
    s9: number;
    s10: number;
    s11: number;
    t3: number;
    t4: number;
    t5: number;
    t6: number;
    pc: number;
    state: State;
}

export enum State
{
    Reset = "Reset",
    Fetch = "Fetch",
    Decode = "Decode",
    Execute = "Execute",
}

export interface CpuStep
{
    step: number;
    fetched_instruction: number;
    decoded_instruction: Instruction;
    cpu_state: CpuState;
    ram_writes: RamWrite[];
}

export type Add = { Add: { rd: number; rs1: number; rs2: number } };
export type Addi = { Addi: { rd: number; rs1: number; imm: number } };
export type Sub = { Sub: { rd: number; rs1: number; rs2: number } };
export type And = { And: { rd: number; rs1: number; rs2: number } };
export type Andi = { Andi: { rd: number; rs1: number; imm: number } };
export type Or = { Or: { rd: number; rs1: number; rs2: number } };
export type Ori = { Ori: { rd: number; rs1: number; imm: number } };
export type Xor = { Xor: { rd: number; rs1: number; rs2: number } };
export type Xori = { Xori: { rd: number; rs1: number; imm: number } };
export type Sll = { Sll: { rd: number; rs1: number; rs2: number } };
export type Slli = { Slli: { rd: number; rs1: number; shamt: number } };
export type Srl = { Srl: { rd: number; rs1: number; rs2: number } };
export type Srli = { Srli: { rd: number; rs1: number; shamt: number } };
export type Sra = { Sra: { rd: number; rs1: number; rs2: number } };
export type Srai = { Srai: { rd: number; rs1: number; shamt: number } };
export type Slt = { Slt: { rd: number; rs1: number; rs2: number } };
export type Slti = { Slti: { rd: number; rs1: number; imm: number } };
export type Sltu = { Sltu: { rd: number; rs1: number; rs2: number } };
export type Sltiu = { Sltiu: { rd: number; rs1: number; imm: number } };
export type Lb = { Lb: { rd: number; rs1: number; offset: number } };
export type Lbu = { Lbu: { rd: number; rs1: number; offset: number } };
export type Sb = { Sb: { rs1: number; rs2: number; offset: number } };
export type Lh = { Lh: { rd: number; rs1: number; offset: number } };
export type Lhu = { Lhu: { rd: number; rs1: number; offset: number } };
export type Sh = { Sh: { rs1: number; rs2: number; offset: number } };
export type Lw = { Lw: { rd: number; rs1: number; offset: number } };
export type Sw = { Sw: { rs1: number; rs2: number; offset: number } };
export type Jal = { Jal: { rd: number; offset: number } };
export type Jalr = { Jalr: { rd: number; rs1: number; offset: number } };
export type Beq = { Beq: { rs1: number; rs2: number; offset: number } };
export type Bne = { Bne: { rs1: number; rs2: number; offset: number } };
export type Blt = { Blt: { rs1: number; rs2: number; offset: number } };
export type Bge = { Bge: { rs1: number; rs2: number; offset: number } };
export type Bltu = { Bltu: { rs1: number; rs2: number; offset: number } };
export type Bgeu = { Bgeu: { rs1: number; rs2: number; offset: number } };
export type Lui = { Lui: { rd: number; imm: number } };
export type Auipc = { Auipc: { rd: number; imm: number } };
export type Fence = { Fence: { pred: number; succ: number } };
export type Ecall = { Ecall: {} };
export type Ebreak = { Ebreak: {} };

export type Instruction =
    | Add
    | Addi
    | Sub
    | And
    | Andi
    | Or
    | Ori
    | Xor
    | Xori
    | Sll
    | Slli
    | Srl
    | Srli
    | Sra
    | Srai
    | Slt
    | Slti
    | Sltu
    | Sltiu
    | Lb
    | Lbu
    | Sb
    | Lh
    | Lhu
    | Sh
    | Lw
    | Sw
    | Jal
    | Jalr
    | Beq
    | Bne
    | Blt
    | Bge
    | Bltu
    | Bgeu
    | Lui
    | Auipc
    | Fence
    | Ecall
    | Ebreak;

export interface RamWrite
{
    addr: number;
    value: number;
}
