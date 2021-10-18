use super::bd::Builder;

#[allow(dead_code)]
struct Director {}

impl Director {
    #[allow(dead_code)]
    fn construct_sports_car(&self, builder: &mut dyn Builder) {
        builder.reset();
        builder.set_seats(2);
        builder.set_engine();
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::car::CarBuilder;
    #[test]
    fn test0() {
        let dir = super::Director {};
        let mut builder = CarBuilder::default();
        dir.construct_sports_car(&mut builder);
        let _car = builder.get_product();
    }
}
