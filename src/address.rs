#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Address(pub u8);

impl Address {
    pub fn to_u8(&self) -> u8 {
        match *self {
            Address(address_) => address_,
        }
    }

    pub fn to_usize(&self) -> usize {
        self.to_u8() as usize
    }
}