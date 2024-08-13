pub mod debug_exit;
pub mod simple_uart;

#[derive(Debug)]
pub struct MmioDeviceBase {
    pub device_name: String,
    pub base_addr: u32,
    pub used_mem_bytes_len: usize,
}

pub enum RequestFromDevice {
    Exit(u8),
}

pub trait MmioDeviceInterface {
    fn poll_request(&mut self) -> Option<RequestFromDevice>;
    fn load8(&self, bytes_offset: usize) -> u8;
    fn store8(&mut self, bytes_offset: usize, value: u8);
    fn load16(&self, bytes_offset: usize) -> u16;
    fn store16(&mut self, bytes_offset: usize, value: u16);
    fn load32(&self, bytes_offset: usize) -> u32;
    fn store32(&self, bytes_offset: usize, value: u32);
    fn is_available_addr(&self, addr: u32) -> bool;
    fn device_name(&self) -> &str;
    fn base_addr(&self) -> u32;
    fn used_mem_bytes_len(&self) -> usize;
}
