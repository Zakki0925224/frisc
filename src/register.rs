use core::fmt::Debug;

#[derive(Clone, Copy, Default)]
pub struct Register(u32);

impl Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

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
