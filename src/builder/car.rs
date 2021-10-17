use super::builder::Builder;

#[derive(Clone, Default)]
pub struct Car {
    engine: String,
    seats: i64,
}

#[derive(Default)]
pub struct CarBuilder{
    car: Car,
}

impl Builder for CarBuilder {
    fn reset(&mut self){
        self.car = Car::default();
    }
    fn set_seats(&mut self, number:i64){
        self.car.seats = number;
    }
    fn set_engine(&mut self){
        self.car.engine = "middle power engine".to_string();
    }
}

impl CarBuilder {
    pub fn get_product(&self) -> Car {
        self.car.clone()
    }
}