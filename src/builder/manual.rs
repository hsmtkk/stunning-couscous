use super::builder::Builder;

#[derive(Clone, Default)]
pub struct Manual {
    engine: String,
    seats: i64,
}

pub struct ManualBuilder {
    manual: Manual,
}

impl Builder for ManualBuilder {
    fn reset(&mut self) {
        self.manual = Manual::default();
    }
    fn set_seats(&mut self, number: i64) {
        self.manual.seats = number;
    }
    fn set_engine(&mut self) {
        self.manual.engine = "high power engine".to_string();
    }
}

impl ManualBuilder {
    fn get_product(&self) -> Manual {
        self.manual.clone()
    }
}
