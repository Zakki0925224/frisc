use clap::Parser;
use frisc::{emulator::Emulator, mmio_device::debug_exit::DebugExit};
use std::fs;
use xmas_elf::{
    header::{Machine, Type},
    ElfFile,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    program_path: String,
    #[arg(long)]
    mem_size: Option<usize>,
    #[arg(long)]
    default_pc: Option<u32>,
    #[arg(long)]
    default_sp: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let bin = fs::read(args.program_path)?;
    let elf = ElfFile::new(&bin).unwrap();
    let elf_header = &elf.header;

    if elf_header.pt1.magic != xmas_elf::header::MAGIC {
        return Err(anyhow::anyhow!("Invalid ELF magic number"));
    }

    if elf_header.pt2.machine().as_machine() != Machine::RISC_V {
        return Err(anyhow::anyhow!("Unsupported machine type"));
    }

    if elf_header.pt2.type_().as_type() != Type::Executable {
        return Err(anyhow::anyhow!("Not executable"));
    }

    println!("{:?}", elf);

    let mut emulator = Emulator::default();
    emulator.register_mmio_device(Box::new(DebugExit::default()));
    emulator.reset();
    emulator.run()?;

    Ok(())
}
