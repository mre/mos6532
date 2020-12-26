use memory::Memory;
use registers::Registers;

pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = CPU::new();
    }

    pub fn set_timer_register(&mut self, value: u8, _interval: u16) {
        self.registers.instat = 0;
        self.registers.intim = value;
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let _cpu = CPU::new();
    }
}
