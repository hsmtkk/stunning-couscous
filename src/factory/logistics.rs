use super::transport::{Ship, Transport, Truck};

pub trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
}

pub struct RoadLogistics {}

impl RoadLogistics {
    #[allow(dead_code)]
    fn new() -> RoadLogistics {
        RoadLogistics {}
    }
}

impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck::new())
    }
}

pub struct SeaLogistics {}

impl SeaLogistics {
    #[allow(dead_code)]
    fn new() -> SeaLogistics {
        SeaLogistics {}
    }
}

impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Logistics;

    #[test]
    fn test_road() {
        let log = super::RoadLogistics::new();
        let trs = log.create_transport();
        trs.deliver();
    }

    #[test]
    fn test_sea() {
        let log = super::SeaLogistics::new();
        let trs = log.create_transport();
        trs.deliver();
    }
}
