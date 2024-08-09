pub const DEFAULT_RAM_SIZE: u32 = 1024 * 1024; // 1MB

#[derive(Debug)]
pub struct Ram(Vec<u8>);

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

    pub fn read8(&self, addr: u32) -> u8 {
        self.0[addr as usize]
    }

    pub fn write8(&mut self, addr: u32, value: u8) {
        self.0[addr as usize] = value;
    }

    pub fn read32(&self, addr: u32) -> u32 {
        let data1 = self.read8(addr);
        let data2 = self.read8(addr + 1);
        let data3 = self.read8(addr + 2);
        let data4 = self.read8(addr + 3);

        // little endian
        (data1 as u32) | ((data2 as u32) << 8) | ((data3 as u32) << 16) | ((data4 as u32) << 24)
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}
