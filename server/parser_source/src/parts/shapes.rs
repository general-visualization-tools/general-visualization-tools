use crate::parts::rect::Rect;
use crate::parts::circle::Circle;

#[derive(Debug)]
enum Shape<'a> {
    Rect(Rect<'a>),
    Circle(Circle<'a>),
}

#[derive(Default, Debug)]
pub struct Shapes<'a> {
    shapes: Vec<Shape<'a>>
}

impl<'a> Shapes<'a> {
    pub fn push_rect(&mut self, rect: Rect<'a>) {
        self.shapes.push(Shape::Rect(rect));

    }
    pub fn push_circle(&mut self, circle: Circle<'a>) {
        self.shapes.push(Shape::Circle(circle));
    }
}

