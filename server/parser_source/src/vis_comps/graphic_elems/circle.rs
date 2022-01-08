use serde::{Serialize};
use std::error::Error;
use core::slice::Iter;
use std::convert::TryFrom;
use crate::context::Context;
use crate::GraphicPartsSetting;
use super::super::graphic_elems::Elem;
use super::super::common_types::{ number::Number, color::Color };
use super::super::traits::{ParsableBasedOnCtx, Visualizable, VisualizableFrom, ConvertableGraphicElem};

#[derive(Serialize, Clone, Debug)]
pub struct Circle<'a>  {
    #[serde(skip)]
    pub(in super::super) group_id: &'a str,
    #[serde(rename="elemID")]
    pub(in super::super) elem_id: &'a str,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theta: Option<Number>,
}

#[derive(Debug)]
struct MultiParametricCircle<'a> {
    group_id: &'a str,
    elem_id:  &'a str,

    color:      Option<Color>,
    grad_begin: Option<Color>,
    grad_end:   Option<Color>,
    grad_ratio: Option<Number>,

    r:     Option<Number>,
    x:     Option<Number>,
    y:     Option<Number>,
    z:     Option<Number>,
    theta: Option<Number>,
}

impl Default for Circle<'_> {
    fn default() -> Self {
        Self {
            group_id: "group0",
            elem_id:  "circle0",

            color: Some(Color::default()),

            r: Some(1.),
            x: Some(0.),
            y: Some(0.),
            z: Some(0.),
            theta: Some(0.),
        }
    }
}

impl Default for MultiParametricCircle<'_> {
    fn default() -> Self {
        Self {
            group_id: "group0",
            elem_id:  "circle0",

            color:      None,
            grad_begin: None,
            grad_end:   None,
            grad_ratio: None,

            r: None,
            x: None,
            y: None,
            z: None,
            theta: None,
        }
    }
}

impl<'a> TryFrom<MultiParametricCircle<'a>> for Circle<'a> {
    type Error = Box<dyn Error>;
    fn try_from(value: MultiParametricCircle<'a>) -> Result<Self, Self::Error> {
        // 主に2つのことを行う
        //   - 存在しないパラメータの補間
        //   - gradationからcolorを作成

        // todo:
        // パラメータの数が多すぎる/少なすぎる場合にどうするかを考える(今は適当に処理
        // 値が不正(rがマイナスなど)の時にどこでチェックをするかを考える

        let mut c = Circle {
            group_id: value.group_id,
            elem_id:  value.elem_id,
            ..Default::default()
        };

        c.r = value.r.or(c.r);
        c.x = value.x.or(c.x);
        c.y = value.y.or(c.y);
        c.z = value.z.or(c.z);
        c.theta = value.theta.or(c.theta);

        c.color = value.color.or(c.color);
        if value.grad_begin.is_some() && value.grad_end.is_some() && value.grad_ratio.is_some() {
            c.color = Some(Color::from_gradation(&value.grad_begin.unwrap(), &value.grad_end.unwrap(), value.grad_ratio.unwrap()));
        }

        Ok(c)
    }
}

impl<'a> Visualizable<'a> for MultiParametricCircle<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "groupID"   => { self.group_id = word; Ok(()) },
            "elemID"    => { self.elem_id  = word; Ok(()) },

            "color"     => { self.color      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradBegin" => { self.grad_begin = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradEnd"   => { self.grad_end   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradRatio" => { self.grad_ratio = Some(word.parse_based_on(ctx)?); Ok(()) },

            "r"         => { self.r          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "x"         => { self.x          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "y"         => { self.y          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "z"         => { self.z          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "theta"     => { self.theta      = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'MultiParametricCircle': {}. please check settings", param_name).into())
        }
    }
}

// todo: default実装飲みなのでimplを書かなくてもいいようにできるかどうか確かめる
impl<'a> VisualizableFrom<'a, MultiParametricCircle<'a>> for Circle<'a> {}

impl<'a> ConvertableGraphicElem<'a> for Circle<'a> {
    fn get_group_id(&self) -> &'a str { self.group_id }
    fn convert_to_elem(self) -> Elem<'a> { Elem::Circle(self) }
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            group_id: self.group_id,
            elem_id: self.elem_id,
            color: if self.color == other.color { None } else { self.color.clone() },
            x:     if self.x     == other.x     { None } else { self.x },
            y:     if self.y     == other.y     { None } else { self.y },
            r:     if self.r     == other.r     { None } else { self.r },
            z:     if self.z     == other.z     { None } else { self.z },
            theta: if self.theta == other.theta { None } else { self.theta },
        }
    }
    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        VisualizableFrom::<'a, MultiParametricCircle<'a>>::from_words_and_setting(words_iter, setting, ctx)
    }
}


