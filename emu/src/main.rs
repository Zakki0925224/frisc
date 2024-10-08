use clap::Parser;
use frisc::{
    emulator::Emulator,
    mmio_device::{debug_exit::DebugExit, simple_uart::SimpleUart},
};
use std::{
    fs::{self, File},
    io::Write,
};
use xmas_elf::{
    header,
    program::{self, ProgramHeader, SegmentData},
    ElfFile,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    program_path: String,
    #[arg(long, short)]
    step_log_path: Option<String>,
    #[arg(long, short)]
    ram_size: Option<usize>,
    #[arg(long, short)]
    default_sp: Option<u32>,
    #[arg(long, short)]
    instruction_log: bool,
}

fn main() -> anyhow::Result<()> {
    let default_stack_size = 1024 * 1024;

    let args = Args::parse();
    let bin = fs::read(args.program_path)?;
    let elf = ElfFile::new(&bin).unwrap();
    let elf_header = &elf.header;

    if elf_header.pt1.magic != header::MAGIC {
        return Err(anyhow::anyhow!("Invalid ELF magic number"));
    }

    if elf_header.pt2.machine().as_machine() != header::Machine::RISC_V {
        return Err(anyhow::anyhow!("Unsupported machine type"));
    }

    if elf_header.pt2.type_().as_type() != header::Type::Executable {
        return Err(anyhow::anyhow!("Not executable"));
    }

    let loadable_phs: Vec<ProgramHeader> = elf.program_iter().filter(|p|p.get_type().unwrap() == program::Type::Load).collect();
    let max_ram_size = loadable_phs
        .iter()
        .map(|ph| ph.virtual_addr() + ph.mem_size())
        .max()
        .expect("Failed to calcurate RAM size") as usize;

    if let Some(ram_size) = args.ram_size {
        if ram_size < max_ram_size {
            return Err(anyhow::anyhow!("RAM size is too small"));
        }
    }

    // 4 bytes alignment
    let mut ram = vec![0u8; (args.ram_size.unwrap_or(max_ram_size) + default_stack_size + 3) & !3];
    for ph in loadable_phs {
        let offset = ph.virtual_addr() as usize;
        let file_size = ph.file_size() as usize;
        let mem_size = ph.mem_size() as usize;
        let data = match ph.get_data(&elf).unwrap() {
            SegmentData::Undefined(data) => data,
            _ => return Err(anyhow::anyhow!("Unsupported segment type")),
        };
        ram[offset..offset + file_size].copy_from_slice(&data[..file_size]);

        if mem_size > file_size {
            ram[offset + file_size..offset + mem_size].fill(0);
        }
    }

    let default_pc = elf_header.pt2.entry_point() as u32;
    let default_sp = args.default_sp.unwrap_or(ram.len() as u32);

    let mut emulator = Emulator::new(ram);
    emulator.register_mmio_device(Box::new(DebugExit::default()));
    emulator.register_mmio_device(Box::new(SimpleUart::default()));
    emulator.reset();
    emulator.cpu.pc.store(default_pc); // pc
    emulator.cpu.x_regs[2].store(default_sp); // sp
    let (exit_code, log) = emulator.run(args.instruction_log)?;
    println!("Exited with 0x{:x}", exit_code);

    if let Some(step_log_path) = args.step_log_path {
        let s = serde_json::to_string(&log)?;
        let mut file = File::create(step_log_path)?;
        file.write_all(s.as_bytes())?;
    }

    Ok(())
}
