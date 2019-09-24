use std::cell::Cell;

#[derive(Debug)]
pub enum Error {
    OutOfRangePin,
}

pub struct Pin<F> {
    providing_power: bool,
    links_providing_power: Cell<i32>,
    raising_edge_events: Cell<Vec<F>>,
    falling_edge_events: Cell<Vec<F>>
}

impl<F> Pin<F> {
    fn set(&mut self, provide_power: bool) {
        if provide_power != self.providing_power {
            self.providing_power = provide_power;

            
        }
    }

    fn link(&mut self, pin: Pin<F>) {
        unimplemented!()
    }

    fn on_raising_edge(&mut self, f: F) {
        self.raising_edge_events.get_mut().push(f);
    }

    fn on_falling_edge(&mut self, f: F) {
        self.falling_edge_events.get_mut().push(f);
    }
}