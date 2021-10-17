pub trait Transport {
    fn deliver(&self);
}

pub struct Truck {}

impl Truck {
    pub fn new() -> Truck {
        Truck {}
    }
}

impl Transport for Truck {
    fn deliver(&self) {
        println!("truck delivery");
    }
}

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }
}

impl Transport for Ship {
    fn deliver(&self) {
        println!("ship delivery");
    }
}
