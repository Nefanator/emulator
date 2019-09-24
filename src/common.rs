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

impl<F> Pin<F>
where F: FnMut()
{
    fn new() -> Self {
        let providing_power = false;
        let links_providing_power = Cell::default();
        let raising_edge_events = Cell::default();
        let falling_edge_events = Cell::default();

        Pin {
            providing_power,
            links_providing_power,
            raising_edge_events,
            falling_edge_events
        }
    }

    fn set(&mut self, provide_power: bool) {
        if provide_power != self.providing_power {
            self.providing_power = provide_power;

            
        }
    }

    fn link(&mut self, pin: Pin<F>) {
        unimplemented!()
    }

    fn on_raising_edge(&mut self, f: F)
    {
        self.raising_edge_events.get_mut().push(f);
    }

    fn on_falling_edge(&mut self, f: F) {
        self.falling_edge_events.get_mut().push(f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut pin1 = Pin::new();
        let mut pin2 = Pin::new();
        let mut value = false;

        pin1.link(pin2);

        pin1.on_raising_edge(move || value = true);
        pin2.set(true);

        assert!(value);
    }
}
