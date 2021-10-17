use super::builder::Builder;

struct Director {
}

impl Director {
    fn construct_sports_car(&self, builder: &mut Builder){
        builder.reset();
        builder.set_seats(2);
        builder.set_engine();
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::car::CarBuilder;
    #[test]
    fn test0(){
        let dir = super::Director{};
        let mut builder = CarBuilder::default();
        dir.construct_sports_car(&mut builder);
        let car = builder.get_product();
    }
}