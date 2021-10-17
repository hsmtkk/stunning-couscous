pub trait Builder {
    fn reset(&mut self);
    fn set_seats(&mut self, number:i64);
    fn set_engine(&mut self);
}
