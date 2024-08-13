use super::{MmioDeviceBase, MmioDeviceInterface};

const DEFAULT_BASE_ADDR: u32 = 0x3f8; // COM1
const DEFAULT_MEM_BYTES_LEN: usize = 5;

#[derive(Debug)]
pub struct SimpleUart {
    device_base: MmioDeviceBase,
}

impl SimpleUart {
    fn new(device_name: String, base_addr: u32, used_mem_bytes_len: usize) -> Self {
        Self {
            device_base: MmioDeviceBase {
                device_name,
                base_addr,
                used_mem_bytes_len,
            },
        }
    }
}

impl MmioDeviceInterface for SimpleUart {
    fn poll_request(&mut self) -> Option<super::RequestFromDevice> {
        None
    }

    fn load8(&self, _bytes_offset: usize) -> u8 {
        0
    }

    fn store8(&mut self, bytes_offset: usize, value: u8) {
        if bytes_offset == 0 {
            print!("{}", value as char);
        }
    }

    fn load16(&self, _bytes_offset: usize) -> u16 {
        0
    }

    fn store16(&mut self, _bytes_offset: usize, _value: u16) {}

    fn load32(&self, _bytes_offset: usize) -> u32 {
        0
    }

    fn store32(&self, _bytes_offset: usize, _value: u32) {}

    fn is_available_addr(&self, addr: u32) -> bool {
        addr >= self.base_addr() && addr < self.base_addr() + self.used_mem_bytes_len() as u32
    }

    fn device_name(&self) -> &str {
        &self.device_base.device_name
    }

    fn base_addr(&self) -> u32 {
        self.device_base.base_addr
    }

    fn used_mem_bytes_len(&self) -> usize {
        self.device_base.used_mem_bytes_len
    }
}

impl Default for SimpleUart {
    fn default() -> Self {
        Self::new(
            String::from("simple-uart"),
            DEFAULT_BASE_ADDR,
            DEFAULT_MEM_BYTES_LEN,
        )
    }
}
