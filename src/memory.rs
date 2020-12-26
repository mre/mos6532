use address::Address;

const ADDR_LO_BARE: u8 = 0x80;
const ADDR_HI_BARE: u8 = 0xFF;

pub const MEMORY_ADDRESS_LO: Address = Address(ADDR_LO_BARE);
pub const MEMORY_ADDRESS_HI: Address = Address(ADDR_HI_BARE);

const MEMORY_SIZE: usize = (ADDR_HI_BARE - ADDR_LO_BARE) as usize + 1usize;

#[derive(Copy)]
pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Clone for Memory {
    fn clone(&self) -> Self {
        *self
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn get_byte(&self, address: Address) -> u8 {
        self.bytes[address.to_usize()]
    }

    pub fn get_byte_mut_ref(&mut self, address: Address) -> &mut u8 {
        &mut self.bytes[address.to_usize()]
    }

    // Sets the byte at the given address to the given value and returns the
    // previous value at the address.
    pub fn set_byte(&mut self, address: Address, value: u8) -> u8 {
        let old_value = self.get_byte(address);
        self.bytes[address.to_usize()] = value;
        old_value
    }

    pub fn set_bytes(&mut self, Address(start): Address, values: &[u8]) {
        let start = start as usize;

        // This panics if the range is invalid
        let end = start + values.len();
        let slice = &mut self.bytes[start..end];

        // JAM: Is this the best way to do this copy?
        for (dest, src) in slice.iter_mut().zip(values.iter()) {
            *dest = *src;
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}
