use memory::Memory;
use registers::Registers;

struct CPU {
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

    pub fn set_timer_register(&mut self, value: u8, interval: u16) {
        self.registers.instat = 0;
        self.registers.intim = value;

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let cpu = CPU::new();
    }
}
