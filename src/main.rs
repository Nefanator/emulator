extern crate wincolor;

mod common;
mod w65c02s;

use w65c02s::W65C02S;
use wincolor::{Console, Color, Intense};

fn main() {
    let mut cpu: w65c02s::Pdip = W65C02S::new();

    cpu.set_address_bus(0b1010_1010_1010_1010).unwrap();
    cpu.set_data_bus(0b0011_0011).unwrap();
    cpu.write_pin(2, true).unwrap();

    let expected = vec!(0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0);

    let mut console = Console::stdout().unwrap();

    for pin in 0..40 {
        if cpu.read_pin(pin).unwrap() != (expected.get(pin).unwrap() == &1) {
            console.fg(Intense::Yes, Color::Red).unwrap();
            println!("Failed: mismatch between the pinout state of the cpu and expected states");
            console.reset().unwrap();
            std::process::exit(0);
        }
    }

    console.fg(Intense::Yes, Color::Green).unwrap();
    println!("Successful!");
    console.reset().unwrap();
}
