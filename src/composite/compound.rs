use super::graphic::Graphic;

pub struct CompoundGraphic {
    graphics: Vec<Box<dyn Graphic>>,
}

impl CompoundGraphic {
    fn new() -> CompoundGraphic {
        let graphics = Vec::new();
        CompoundGraphic{graphics}
    }

    fn add(&mut self, child: Box<dyn Graphic>) {
        self.graphics.push(child);
    }
}

impl Graphic for CompoundGraphic {
    fn mv(&mut self, x:i32, y:i32) {
        for mut g in self.graphics {
            g.mv(x, y);
        }
    }

    fn draw(&self){
        for g in &self.graphics {
            g.draw();
        }
    }
}