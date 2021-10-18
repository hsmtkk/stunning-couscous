use crate::adapter::round_peg::Circle;
use crate::adapter::square_peg::SquarePeg;

pub struct SquarePegAdapter {
    peg: Box<SquarePeg>,
}

impl SquarePegAdapter {
    #[allow(dead_code)]
    pub fn new(peg: Box<SquarePeg>) -> SquarePegAdapter {
        SquarePegAdapter{peg}
    }
}

impl Circle for SquarePegAdapter {
    fn get_radius(&self) -> f64 {
        let w: f64 = self.peg.get_width() as f64;
        w * 2.0_f64.sqrt() / 2.0
    }
}