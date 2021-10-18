use super::round_peg::Circle;

#[allow(dead_code)]
pub struct RoundHole {
    radius: f64,
}

impl RoundHole {
    #[allow(dead_code)]
    pub fn new(radius:f64) -> RoundHole {
        RoundHole{radius}
    }

    #[allow(dead_code)]
    pub fn fits(&self, peg: impl Circle) -> bool {
        self.get_radius() >= peg.get_radius()
    }

    #[allow(dead_code)]
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}