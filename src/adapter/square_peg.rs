pub struct SquarePeg {
    width :f64,
}

impl SquarePeg {
    #[allow(dead_code)]
    pub fn new(width:f64) -> SquarePeg {
        SquarePeg{width}
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }
}