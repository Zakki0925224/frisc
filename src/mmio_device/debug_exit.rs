use super::{MmioDeviceBase, MmioDeviceInterface, RequestFromDevice};

const DEFAULT_BASE_ADDR: u32 = 0xf4;
const DEFAULT_MEM_BYTES_LEN: usize = 1;

#[derive(Debug)]
pub struct DebugExit {
    device_base: MmioDeviceBase,
    exit_code: Option<u8>,
}

impl DebugExit {
    fn new(device_name: String, base_addr: u32, used_mem_bytes_len: usize) -> Self {
        Self {
            device_base: MmioDeviceBase {
                device_name,
                base_addr,
                used_mem_bytes_len,
            },
            exit_code: None,
        }
    }
}

impl MmioDeviceInterface for DebugExit {
    fn poll_request(&mut self) -> Option<RequestFromDevice> {
        if let Some(exit_code) = self.exit_code {
            return Some(RequestFromDevice::Exit(exit_code));
        }

        None
    }

    fn load8(&self, _bytes_offset: usize) -> u8 {
        0
    }

    fn store8(&mut self, bytes_offset: usize, value: u8) {
        if bytes_offset == 0 {
            println!("[{}]: Exited with 0x{:x}", self.device_name(), value);
            self.exit_code = Some(value);
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

impl Default for DebugExit {
    fn default() -> Self {
        Self::new(
            String::from("debug-exit"),
            DEFAULT_BASE_ADDR,
            DEFAULT_MEM_BYTES_LEN,
        )
    }
}
