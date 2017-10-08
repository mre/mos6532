// TODO:
//TIM1T   11111111  set 1 clock interval (838 nsec/interval)
//TIM8T   11111111  set 8 clock interval (6.7 usec/interval)
//TIM64T  11111111  set 64 clock interval (53.6 usec/interval)
//T1024T  11111111  set 1024 clock interval (858.2 usec/interval)

use rand;
type Register = u8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Registers {
    /// Port A; input or output  (read or write)
    pub swcha: Register,
    /// Port A DDR, 0=input, 1=output
    pub swacnt: Register,
    /// Port B; console switches (read only)
    pub swchb: Register,
    /// Port B DDR (hardwired as input)
    pub swbcnt: Register,
    // "Interval Timer" used for timer output (read only)
    pub intim: Register,
    // Timer Status (also known as Interrupt Flag Register).
    // Used for checking overflows.
    // Only two bits are relevant
    // Bit 7: Timer flag (cleared when timer register is written or read)
    // Bit 6: PA7 flag (cleared when the Interrupt flag register is read)
    pub instat: Register,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            swcha: 0,
            swacnt: 0,
            swchb: 0,
            swbcnt: 0,
            intim: rand::random::<(u8)>(),
            instat: 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_registers() {
        let mut registers = Registers::new();
        registers.swcha = 0b11111111;
        assert_eq!(registers.swcha, 255);
    }
}
