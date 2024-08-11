use std::fmt::Debug;

pub const DEFAULT_RAM_SIZE: u32 = 1024 * 1024; // 1MB

pub struct Ram(Vec<u8>);

impl Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Ram(size: {}):", self.0.len())?;

        // hexdump
        let ram = self.0.as_slice();
        for i in 0..=ram.len() / 16 {
            write!(f, "{:08x}", i * 16)?;
            let slice = &ram[i * 16..(i * 16 + 16).min(ram.len())];
            for (j, b) in slice.iter().enumerate() {
                if j % 8 == 0 {
                    write!(f, " ")?;
                }

                write!(f, "{:02x}", b)?;
            }

            if slice.len() < 16 {
                for _ in 0..16 - slice.len() {
                    write!(f, "    ")?;
                }
                write!(f, " ")?;
            }

            write!(f, " |")?;
            for b in slice {
                if *b >= 0x20 && *b <= 0x7e {
                    write!(f, "{}", *b as char)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }

        writeln!(f, "")
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new(DEFAULT_RAM_SIZE)
    }
}

impl Ram {
    pub fn new(size: u32) -> Self {
        Self(vec![0; size as usize])
    }

    pub fn new_with_data(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn load8(&self, addr: u32) -> u8 {
        self.0[addr as usize]
    }

    pub fn store8(&mut self, addr: u32, value: u8) {
        self.0[addr as usize] = value;
    }

    pub fn load32(&self, addr: u32) -> u32 {
        let data1 = self.load8(addr);
        let data2 = self.load8(addr + 1);
        let data3 = self.load8(addr + 2);
        let data4 = self.load8(addr + 3);

        // little endian
        (data1 as u32) | ((data2 as u32) << 8) | ((data3 as u32) << 16) | ((data4 as u32) << 24)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}
