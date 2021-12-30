use std::error::Error;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use super::traits::FromStrBasedOnCtx;
use super::basic_types::Number;
use crate::context::Context;


#[derive(Debug, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from_gradation(begin: &Color, end: &Color, ratio: Number) -> Color {
        Color {
            r: (begin.r as f64 + (end.r as f64 - begin.r as f64) * ratio) as u8,
            g: (begin.g as f64 + (end.g as f64 - begin.g as f64) * ratio) as u8,
            b: (begin.b as f64 + (end.b as f64 - begin.b as f64) * ratio) as u8,
        }
    }
}

impl Default for Color { fn default() -> Self { Self { r: 255, g: 255, b: 255 } } }

impl FromStrBasedOnCtx for Color {
    fn from_str_based_on(s: &str, _ctx: &Context) -> Result<Self, Box<dyn Error>> {
        if s.len() == 7 && s.starts_with('#') {
            Ok(Color {
                r: u8::from_str_radix(&s[5..7], 16)?,
                g: u8::from_str_radix(&s[3..5], 16)?,
                b: u8::from_str_radix(&s[1..3], 16)?
            })
        } else {
            Ok(Palette::get(s)?)
        }
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer
    {
        let mut state = serializer.serialize_struct("Color", 1)?;
        state.serialize_field("color", &format!("#{:x}{:x}{:x}", self.r, self.g, self.b))?;
        state.end()
    }
}

pub struct Palette {}

impl Palette {
    pub fn white() -> Color { Color { r:   0, g:   0, b:   0 } }
    pub fn black() -> Color { Color { r: 255, g: 255, b: 255 } }
    pub fn green() -> Color { Color { r:   0, g: 163, b: 129 } }
    pub fn blue()  -> Color { Color { r:  39, g:  74, b: 120 } }
    pub fn red()   -> Color { Color { r: 201, g:  23, b:  30 } }

    pub fn get(s: &str) -> Result<Color, Box<dyn Error>> {
        match s.to_lowercase().as_str() {
            "white" => Ok(Self::white()),
            "black" => Ok(Self::black()),
            "green" => Ok(Self::green()),
            "blue"  => Ok(Self::blue()),
            "red"   => Ok(Self::red()),
            _ => Err(format!("failed to convert string to color: {}", s).into())
        }
    }
}
