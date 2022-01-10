use serde::{Serialize};

use super::super::unique_id_generator::UID;
use super::deleter::ElemDeleter;
use super::camera::Camera;
use super::circle::Circle;
use super::path::Path;
use super::rect::Rect;

// you can create trait object only from object safety trait.
// trait object lost its type info,
// so it can't dispatch a func, that takes Self(/=self) as an argument for example.
// about Object Safety: https://doc.rust-lang.org/reference/items/traits.html#object-safety
// use enum instead of trait object: https://bennetthardwick.com/dont-use-boxed-trait-objects-for-struct-internals/

// if you use enum class, there are many redundant pattern matches,
// but it is also hard to use trait object in this case, probably.
// after all, i use enum class, because it is very easy to persuade the compiler.

pub trait ElementTrait<'a>
where
    Self: Sized
{
    fn convert_to_elem(self) -> Elem<'a>;
    fn extract_diff_from(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "elemType")]
pub enum Elem<'a> {
    Camera(Camera<'a>),
    Circle(Circle<'a>),
    Path(Path<'a>),
    Rect(Rect<'a>),
}

impl<'a> Elem<'a> {
    pub fn get_unique_id(&self) -> UID {
        match self {
            Elem::Camera(x) => x.unique_id,
            Elem::Circle(x) => x.unique_id,
            Elem::Path(x) => x.unique_id,
            Elem::Rect(x) => x.unique_id,
        }
    }
    pub fn set_unique_id(&mut self, uid: UID) {
        match self {
            Elem::Camera(x) => x.unique_id = uid,
            Elem::Circle(x) => x.unique_id = uid,
            Elem::Rect(x) => x.unique_id = uid,
            Elem::Path(x) => x.unique_id = uid,
        };
    }
    pub fn get_elem_id(&self) -> (&str, &str) {
        match self {
            Elem::Camera(_) => ("Camera", ""),
            Elem::Circle(x) => ("Circle", x.name),
            Elem::Path(x) => ("Path", x.name),
            Elem::Rect(x) => ("Rect", x.name),
        }
    }
    pub fn get_this_elem_deleter(&self) -> ElemDeleter {
        ElemDeleter { unique_id: self.get_unique_id() }
    }
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
            (_,_) => unreachable!("extract_diff_from's argument is invalid")
        }
    }
}
