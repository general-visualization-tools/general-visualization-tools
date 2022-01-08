use serde::{ Serialize };
use super::number::Number;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Point { pub x: Number, pub y: Number }
