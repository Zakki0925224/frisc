use crate::{
    cpu::Cpu,
    mmio_device::{MmioDeviceInterface, RequestFromDevice},
    ram::Ram,
    step_log,
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

    pub fn run(&mut self) -> anyhow::Result<(u8, step_log::Log)> {
        let mut log = step_log::Log {
            init_cpu_state: step_log::CpuStateLog::new(&self.cpu),
            init_ram: self.ram.0.clone(),
            steps: Vec::new(),
            dev_reqs: Vec::new(),
        };

        let mut exit_code = 0;

        'a: loop {
            for mmio_device in &mut self.mmio_devices {
                let req = match mmio_device.poll_request() {
                    Some(request) => request,
                    None => continue,
                };

                log.dev_reqs.push(step_log::DeviceRequest {
                    step: self.cpu.step,
                    req: req.clone(),
                });

                match req {
                    RequestFromDevice::Exit(exit_code_) => {
                        exit_code = exit_code_;
                        break 'a;
                    }
                }
            }

            let step_log = self
                .cpu
                .fetch_decode_execute(&mut self.ram, &mut self.mmio_devices)?;
            log.steps.push(step_log);

            if self.cpu.pc.load() as usize >= self.ram.size() {
                break;
            }
        }

        Ok((exit_code, log))
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
