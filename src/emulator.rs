use crate::{
    cpu::Cpu,
    mmio_device::{MmioDeviceInterface, RequestFromDevice},
    ram::Ram,
};
use std::fmt::Debug;

#[derive(Default)]
pub struct Emulator {
    pub cpu: Cpu,
    pub ram: Ram,
    pub mmio_devices: Vec<Box<dyn MmioDeviceInterface>>,
}

impl Debug for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emulator")
            .field("cpu", &self.cpu)
            .field("ram", &self.ram)
            .finish()
    }
}

impl Emulator {
    pub fn new(ram_data: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::default(),
            ram: Ram::new_with_data(ram_data),
            mmio_devices: Vec::new(),
        }
    }

    pub fn register_mmio_device(&mut self, device: Box<dyn MmioDeviceInterface>) {
        self.mmio_devices.push(device);
    }

    pub fn run(&mut self) -> anyhow::Result<u8> {
        let mut exit_code = 0;

        'a: loop {
            for mmio_device in &mut self.mmio_devices {
                let request = match mmio_device.poll_request() {
                    Some(request) => request,
                    None => continue,
                };

                match request {
                    RequestFromDevice::Exit(exit_code_) => {
                        exit_code = exit_code_;
                        break 'a;
                    }
                }
            }

            self.cpu
                .fetch_decode_execute(&mut self.ram, &mut self.mmio_devices)?;
            if self.cpu.pc.load() as usize >= self.ram.size() {
                break;
            }
        }

        Ok(exit_code)
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
