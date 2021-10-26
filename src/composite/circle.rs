use super::dot::Dot;
use super::graphic::Graphic;

pub struct Circle {
    center: Dot,
    radius: u32,
}

impl Circle {
    pub fn new(x:i32, y:i32, radius:u32) -> Circle {
        let center = Dot::new(x, y);
        Circle{center, radius}
    }
}

impl Graphic for Circle {
    fn mv(&mut self, x:i32, y:i32){
        self.center.mv(x, y);
    }

    fn draw(&self) {
        self.center.draw();
        println!("drawing circle at radius={}", self.radius);
    }
}