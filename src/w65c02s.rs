use std::cell::Cell;
use super::common::Error;

const MAX_PIN_COUNT: usize = 44;

pub struct Pdip { 
    common: Common
}

struct Common {
    pins: Vec<(bool, Cell<i32>)>,
    pin_count: usize
}

pub trait W65C02S {
    fn new() -> Self;

    fn read_pin(&self, pin: usize) -> Result<bool, Error>;

    fn write_pin(&mut self, pin: usize, state: bool) -> Result<(), Error>;

    fn set_address_bus(&mut self, address: u16) -> Result<(), Error>;

    fn set_data_bus(&mut self, data: u8) -> Result<(), Error>;
}

impl W65C02S for Pdip {
    fn new() -> Pdip {
        let mut common = Common::new();
        common.pin_count = 40;
        
        for pin in 0..40 {
            common.pins.push((false, Cell::new(0)));
        }

        Pdip { common }
    }

    fn read_pin(&self, pin: usize) -> Result<bool, Error> {
        self.common.read_pin(pin)
    }

    fn write_pin(&mut self, pin: usize, state: bool) -> Result<(), Error> {
        self.common.write_pin(pin, state)
    }

    #[allow(dead_code)]
    fn set_address_bus(&mut self, address: u16) -> Result<(), Error> {
        for address_bit in 0..=15 {
            let pin = if address_bit < 12 {
                address_bit + 9
            } else {
                address_bit + 10
            };

            let pin = &mut self.common.pins[pin];
            
            let is_high = pin.0;
            let go_high = address >> address_bit & 1 == 1;

            if is_high != go_high {
                pin.0 = !is_high;
                
                let offset = if is_high { -1 } else { 1 };
                pin.1.set(pin.1.get() + offset);
            }            
        }
        
        Ok(())        
    }

    #[allow(dead_code)]
    fn set_data_bus(&mut self, data: u8) -> Result<(), Error> {
        for data_bit in 0..=7 {
            let pin = 33 - data_bit;

            let pin = &mut self.common.pins[pin];
            
            let is_high = pin.0;
            let go_high = data >> data_bit & 1 == 1;

            if is_high != go_high {
                pin.0 = !is_high;
                
                let offset = if is_high { -1 } else { 1 };
                pin.1.set(pin.1.get() + offset);
            }            
        }
        
        Ok(())      
    }
}

impl W65C02S for Common {
    fn new() -> Self { 
        let pins = vec!();
        let pin_count = MAX_PIN_COUNT;

        Common { pins, pin_count }
    }

    fn read_pin(&self, pin: usize) -> Result<bool, Error> {
        if let Some(state) = self.pins.get(pin) {
            Ok(state.0)
        } else {
            Err(Error::OutOfRangePin)
        }
    }

    fn write_pin(&mut self, pin: usize, state: bool) -> Result<(), Error> {
        let new_state = state;
        
        if let Some(state) = self.pins.get_mut(pin) {
            if new_state != state.0 {
                state.0 = new_state;
                state.1.set(state.1.get() + if new_state { 1 } else { -1 });
            }

            Ok(())            
        } else {
            Err(Error::OutOfRangePin)
        }
    }

    fn set_address_bus(&mut self, _address: u16) -> Result<(), Error> { panic!("common has no pinout defined") }

    fn set_data_bus(&mut self, _data: u8) -> Result<(), Error> { panic!("common has no pinout defined") }
}