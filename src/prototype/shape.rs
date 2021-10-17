pub struct Shape {
    x: i64,
    y: i64,
    color: String,
}

impl Shape {
    fn new(x:i64, y:i64, color:&str) -> Shape {
        Shape{x,y, color:color.to_string()}
    }
}

pub trait Cloneable {
    fn clone(&self) -> Box<dyn Cloneable>;
}

pub struct Circle {
    shape: Shape,
    radius: i64,
}

impl Circle {
    fn new(x:i64, y:i64, radius:i64) -> Circle {
        let shape = Shape::new(x, y, "blue");
        Circle{shape, radius}
    }
}

impl Cloneable for Circle {
    fn clone(&self) -> Box<dyn Cloneable> {
        Box::new(Circle::new(self.shape.x, self.shape.y, self.radius))
    }
}

#[cfg(test)]
mod tests {
    use crate::prototype::shape::Cloneable;
    #[test]
    fn test0(){
        let circle0 = super::Circle::new(0, 1, 2);
        let circle1 = circle0.clone();
    }
}