pub trait Circle {
    fn get_radius(&self) -> f64;
}

pub struct RoundPeg {
    radius: f64,
}

impl RoundPeg {
    #[allow(dead_code)]
    pub fn new(radius:f64) -> RoundPeg {
        RoundPeg{radius}
    }
}

impl Circle for RoundPeg {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}