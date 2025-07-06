#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
    address: u32
}

impl Address {
    pub fn new(address: u32) -> Self {
        if address > 16_777_215 {
            panic!("Address {} out of range, expected 0-16,777,215", address);
        }

        Self { address }
    }

    pub fn address(&self) -> u32 {
        self.address
    }
    
    pub fn set_address(&mut self, address: u32) {
        if address > 16_777_215 {
            panic!("Address {} out of range, expected 0-16,777,215", address);
        }

        self.address = address;
    }
}