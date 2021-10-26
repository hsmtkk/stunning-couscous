use super::graphic::Graphic;

pub struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    pub fn new(x:i32, y:i32) -> Dot {
        Dot{x,y}
    }
}

impl Graphic for Dot {
    fn mv(&mut self ,x: i32, y:i32) {
        self.x += x;
        self.y += y;
    }

    fn draw(&self){
        println!("drawing dot at x={} y={}", self.x, self.y);
    }
}
