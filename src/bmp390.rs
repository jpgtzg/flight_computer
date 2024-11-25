// Written by Juan Pablo Gutiérrez

pub struct BMP390 {
    address: u8,
}

impl BMP390 {
    pub fn new(address: u8) -> Self {
        Self { address }
    }
}
