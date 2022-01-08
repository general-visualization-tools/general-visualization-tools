use serde::{Serialize};
use std::slice::Iter;
use std::error::Error;
use crate::context::Context;
use crate::setting::GraphicPartsSetting;
use super::super::common_types::number::Number;
use super::super::traits::{Visualizable, ParsableBasedOnCtx, ConvertableGraphicElem};
use super::Elem;

#[derive(Debug, Clone, Serialize)]
pub struct Camera<'a> {
    #[serde(skip)]
    pub(in super::super) group_id: &'a str,
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

impl<'a> ConvertableGraphicElem<'a> for Camera<'a> {
    fn get_group_id(&self) -> &'a str { self.group_id }
    fn convert_to_elem(self) -> Elem<'a> { Elem::Camera(self) }
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            group_id: self.group_id,
            x: if self.x == other.x { None } else { self.x },
            y: if self.y == other.y { None } else { self.y },
            w: if self.w == other.w { None } else { self.w },
            h: if self.h == other.h { None } else { self.h },
        }
    }
    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        <Camera<'a> as Visualizable<'a>>::from_words_and_setting(words_iter, setting, ctx)
    }
}
