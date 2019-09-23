const PIN_COUNT: usize = 40;

use super::common::Error;

pub struct Mos6502 {
    pins: [bool; PIN_COUNT]
}

impl Mos6502 {
    pub fn new() -> Result<Mos6502, Error> {
        let pins = [false; PIN_COUNT];
        Ok(Mos6502{pins})
    }

    pub fn read_pin(&self, pin: usize) -> Result<&bool, Error> {
        if pin > PIN_COUNT {
            return Err(Error::OutOfRangePin);
        }

        Ok(&self.pins[pin])
    }

    pub fn write_pin(&mut self, pin: usize, state: bool) -> Result<(), Error> {
        if pin > PIN_COUNT {
            return Err(Error::OutOfRangePin);
        }

        self.pins[pin] = state;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn set_address(&mut self, address: u16) -> Result<(), Error> {
        for address_bit in 0..=15 {
            let pin = if address_bit < 12 {
                address_bit + 9
            } else {
                address_bit + 10
            };
            
            self.pins[pin] = (address >> address_bit) & 1 == 1;
        }
        
        Ok(())        
    }

    #[allow(dead_code)]
    pub fn set_data(&mut self, data: u8) -> Result<(), Error> {
        for data_bit in 0..=7 {
            let pin = 33 - data_bit;
            
            self.pins[pin] = (data >> data_bit) & 1 == 1;
        }
        
        Ok(())        
    }
}
