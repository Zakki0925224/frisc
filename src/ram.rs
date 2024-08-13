use std::fmt::Debug;

use crate::mmio_device::MmioDeviceInterface;

pub const DEFAULT_RAM_SIZE: u32 = 1024 * 1024; // 1MB

pub struct Ram(pub Vec<u8>);

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

    pub fn load16(&self, addr: u32) -> u16 {
        let addr = addr as usize;
        let data1 = self.0[addr];
        let data2 = self.0[addr + 1];

        // little endian
        u16::from_le_bytes([data1, data2])
    }

    pub fn store16(&mut self, addr: u32, value: u16) {
        let bytes = value.to_le_bytes();
        let addr = addr as usize;
        self.0[addr] = bytes[0];
        self.0[addr + 1] = bytes[1];
    }

    pub fn load32(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        let data1 = self.0[addr];
        let data2 = self.0[addr + 1];
        let data3 = self.0[addr + 2];
        let data4 = self.0[addr + 3];

        u32::from_le_bytes([data1, data2, data3, data4])
    }

    pub fn store32(&mut self, addr: u32, value: u32) {
        let bytes = value.to_le_bytes();
        let addr = addr as usize;
        self.0[addr] = bytes[0];
        self.0[addr + 1] = bytes[1];
        self.0[addr + 2] = bytes[2];
        self.0[addr + 3] = bytes[3];
    }

    pub fn load8_with_mmio(
        &self,
        addr: u32,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) -> u8 {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                return mmio_device.load8(bytes_offset);
            }
        }

        return self.load8(addr);
    }

    pub fn store8_with_mmio(
        &mut self,
        addr: u32,
        value: u8,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                mmio_device.store8(bytes_offset, value);
                return;
            }
        }

        self.store8(addr, value);
    }

    pub fn load16_with_mmio(
        &self,
        addr: u32,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) -> u16 {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                return mmio_device.load16(bytes_offset);
            }
        }

        return self.load16(addr);
    }

    pub fn store16_with_mmio(
        &mut self,
        addr: u32,
        value: u16,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                mmio_device.store16(bytes_offset, value);
                return;
            }
        }

        self.store16(addr, value);
    }

    pub fn load32_with_mmio(
        &self,
        addr: u32,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) -> u32 {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                return mmio_device.load32(bytes_offset);
            }
        }

        return self.load32(addr);
    }

    pub fn store32_with_mmio(
        &mut self,
        addr: u32,
        value: u32,
        mmio_devices: &mut Vec<Box<dyn MmioDeviceInterface>>,
    ) {
        for mmio_device in mmio_devices {
            if mmio_device.is_available_addr(addr) {
                let bytes_offset = (addr - mmio_device.base_addr()) as usize;
                mmio_device.store32(bytes_offset, value);
                return;
            }
        }

        self.store32(addr, value);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}
