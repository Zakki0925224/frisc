#[derive(Debug, Clone, Copy, Default)]
pub struct Register(u32);

impl Register {
    pub fn load(&self) -> u32 {
        self.0
    }

    pub fn store(&mut self, value: u32) {
        self.0 = value
    }
}

pub type ProgramCounter = Register;

impl ProgramCounter {
    pub fn increment(&mut self) {
        if self.0 == u32::MAX {
            self.0 = 0;
            return;
        }

        self.0 += 4
    }
}
