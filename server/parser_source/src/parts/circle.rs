use serde::{Serialize};
use std::error::Error;
use std::convert::TryFrom;

use super::basic_types::Number;
use super::color::{ Color };
use super::traits::{ParsableBasedOnCtx, Visualizable, VisualizableFrom};
use crate::context::Context;

#[derive(Serialize, Clone, Debug)]
pub struct Circle<'a>  {
    #[serde(skip)]
    canvas_id: &'a str,
    #[serde(rename="shapeID")]
    shape_id: &'a str,

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
pub struct MultiParametricCircle<'a> {
    canvas_id: &'a str,
    shape_id:  &'a str,

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
            canvas_id: "canvas0",
            shape_id: "circle0",

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
            canvas_id: "canvas0",
            shape_id: "circle0",

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
            canvas_id: value.canvas_id,
            shape_id : value.shape_id,
            ..Default::default()
        };

        c.r = value.r.or(c.r);
        c.x = value.x.or(c.x);
        c.y = value.y.or(c.y);
        c.z = value.z.or(c.z);
        c.theta = value.theta.or(c.theta);

        Ok(c)
    }
}

impl<'a> Visualizable<'a> for MultiParametricCircle<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "canvasID"  => { self.canvas_id  = word;                            Ok(()) },
            "shapeID"   => { self.shape_id   = word;                            Ok(()) },

            "color"     => { self.color      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradBegin" => { self.grad_begin = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradEnd"   => { self.grad_end   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradRatio" => { self.grad_ratio = Some(word.parse_based_on(ctx)?); Ok(()) },

            "r"         => { self.r          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "x"         => { self.x          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "y"         => { self.y          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "z"         => { self.z          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "theta"     => { self.theta      = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'MultiParametricRect': {}. please check settings", param_name).into())
        }
    }
}

impl<'a> VisualizableFrom<'a, MultiParametricCircle<'a>> for Circle<'a> {}
