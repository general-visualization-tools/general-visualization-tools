use serde::{Serialize};
use std::error::Error;
use crate::context::Context;
use super::super::unique_id_generator::UID;
use super::super::common_types::number::Number;
use super::super::traits::{ Visualizable, ParsableBasedOnCtx };
use super::elems::{ Elem, ElementTrait };

#[derive(Debug, Clone, Serialize)]
pub struct Camera<'a> {
    #[serde(flatten)]
    pub (in super) unique_id: UID,
    #[serde(skip)]
    pub group_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    w: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    h: Option<Number>,
}

impl<'a> Default for Camera<'a> {
    fn default() -> Self {
        Self {
            unique_id: UID::unset(),
            group_id: "group0",
            x: Some(0.),
            y: Some(0.),
            w: Some(1000.),
            h: Some(1000.)
        }
    }
}

impl<'a> Visualizable<'a> for Camera<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "groupID" => { self.group_id = word; Ok(()) },
            "x" => { self.x = Some(word.parse_based_on(ctx)?); Ok(()) },
            "y" => { self.y = Some(word.parse_based_on(ctx)?); Ok(()) },
            "w" => { self.w = Some(word.parse_based_on(ctx)?); Ok(()) },
            "h" => { self.h = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'Camera': {}. please check settings", param_name).into())
        }
    }
}

impl<'a> ElementTrait<'a> for Camera<'a> {
    fn convert_to_elem(self) -> Elem<'a> { Elem::Camera(self) }
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            unique_id: self.unique_id,
            group_id: if self.group_id == other.group_id { "" } else { self.group_id },
            x: if self.x == other.x { None } else { self.x },
            y: if self.y == other.y { None } else { self.y },
            w: if self.w == other.w { None } else { self.w },
            h: if self.h == other.h { None } else { self.h },
        }
    }
}
