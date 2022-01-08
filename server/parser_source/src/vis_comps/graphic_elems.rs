pub mod camera;
pub mod circle;
pub mod rect;
pub mod path;


use serde::{ Serialize };
pub use rect::Rect;
pub use path::Path;
pub use circle::Circle;
pub use camera::Camera;
use super::traits::ConvertableGraphicElem;

#[derive(Debug, Clone, Serialize)]
pub struct ForDeletion<'a> {
    #[serde(rename="elemID")]
    elem_id: &'a str
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "elemType")]
pub enum Elem<'a> {
    ForDeletion(ForDeletion<'a>),
    Camera(Camera<'a>),
    Circle(Circle<'a>),
    Path(Path<'a>),
    Rect(Rect<'a>),
}

impl<'a> Elem<'a> {
    pub fn get_elem_id(&self) -> &'a str {
        match self {
            Elem::ForDeletion(d) => d.elem_id,
            Elem::Circle(c) => c.elem_id,
            Elem::Path(p) => p.elem_id,
            Elem::Rect(r) => r.elem_id,

            Elem::Camera(_) => "Camera"
        }
    }
    pub fn get_elem_for_deletion(&self) -> Elem { Elem::ForDeletion(ForDeletion { elem_id: self.get_elem_id() }) }
    pub fn extract_diff_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Elem::Camera(camera), Elem::Camera(other)) => {
                Elem::Camera(camera.extract_diff_from(other))
            }
            (Elem::Circle(circle), Elem::Circle(other)) => {
                Elem::Circle(circle.extract_diff_from(other))
            },
            (Elem::Rect(rect), Elem::Rect(other)) => {
                Elem::Rect(rect.extract_diff_from(other))
            },
            (Elem::Path(path), Elem::Path(other)) => {
                Elem::Path(path.extract_diff_from(other))
            }
            (_,_) => unreachable!("elems isn't same")
        }
    }
}


