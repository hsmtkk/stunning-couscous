pub trait Transport {
    fn deliver(&self);
}

pub struct Truck {}

impl Transport for Truck {

}

pub struct Ship {}

impl Transport for Ship {
    
}
