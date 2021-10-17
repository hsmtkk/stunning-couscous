pub trait Chair {
    fn has_legs(&self) -> bool;
    fn sit_on(&self);
}

#[derive(Default)]
pub struct VictorianChair {}

impl Chair for VictorianChair {
    fn has_legs(&self) -> bool {
        true
    }

    fn sit_on(&self) {
        println!("this is a victorian chair");
    }
}

#[derive(Default)]
pub struct ModernChair {}

impl Chair for ModernChair {
    fn has_legs(&self) -> bool {
        false
    }

    fn sit_on(&self) {
        println!("this is a modern chair");
    }
}
