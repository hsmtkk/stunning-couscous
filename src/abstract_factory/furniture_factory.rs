use super::chair::{Chair, ModernChair, VictorianChair};

trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
}

#[derive(Default)]
struct VictorianFurnitureFactory {}

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair::default())
    }
}

#[derive(Default)]
struct ModernChairFactory {}

impl FurnitureFactory for ModernChairFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::abstract_factory::furniture_factory::FurnitureFactory;
    #[test]
    fn test_victorian() {
        let fac = super::VictorianFurnitureFactory::default();
        let ch = fac.create_chair();
        ch.sit_on();
    }

    #[test]
    fn test_modern() {
        let fac = super::ModernChairFactory::default();
        let ch = fac.create_chair();
        ch.sit_on();
    }
}
