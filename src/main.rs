mod common;
mod components;

fn main() {
    let mut cpu = components::Mos6502::new().unwrap();

    cpu.set_address(0b1111_0000_0000_0000).unwrap();
    cpu.set_data(0b0011_0011).unwrap();
    cpu.write_pin(2, true).unwrap();

    for pin in 0..40 {
        let state = if *cpu.read_pin(pin).unwrap() {
            1
        } else {
            0
        };

        println!("{}: {}, ", pin, state);
    }
}
