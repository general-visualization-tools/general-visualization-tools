use serde::{Serialize};
use std::error::Error;
use std::convert::TryFrom;

use super::basic_types::Number;
use super::color::{ Color };
use super::traits::{ParsableBasedOnCtx, Visualizable, VisualizableFrom};
use crate::context::Context;

#[derive(Serialize, Clone, Debug)]
pub struct Rect<'a> {
    #[serde(skip)]
    pub(in super) canvas_id: &'a str,
    #[serde(rename="shapeID")]
    pub(in super) shape_id: &'a str,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    w: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    h: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theta: Option<Number>,
}

#[derive(Debug)]
pub struct MultiParametricRect<'a> {
    canvas_id: &'a str,
    shape_id:  &'a str,

    color:      Option<Color>,
    grad_begin: Option<Color>,
    grad_end:   Option<Color>,
    grad_ratio: Option<Number>,

    left:     Option<Number>,
    right:    Option<Number>,
    width:    Option<Number>,
    center_x: Option<Number>,

    top:      Option<Number>,
    bottom:   Option<Number>,
    height:   Option<Number>,
    center_y: Option<Number>,

    z:     Option<Number>,
    theta: Option<Number>,
}

impl Default for Rect<'_> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            shape_id : "rect0",
            color: Some(Default::default()),
            x: Some(0.),
            y: Some(0.),
            w: Some(1.),
            h: Some(1.),
            z: Some(0.),
            theta: Some(0.),
        }
    }
}

impl Default for MultiParametricRect<'_> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            shape_id : "rect0",

            color     : Some(Color::default()),
            grad_begin: None,
            grad_end  : None,
            grad_ratio: None,

            left  : None,
            top   : None,
            width : None,
            height: None,

            right   : None,
            bottom  : None,
            center_x: None,
            center_y: None,

            theta: None,
            z: None,
        }
    }
}

impl<'a> TryFrom<MultiParametricRect<'a>> for Rect<'a> {
    type Error = Box<dyn Error>;
    fn try_from(value: MultiParametricRect<'a>) -> Result<Self, Self::Error> {
        // 主に3つのことを行う
        //   - 存在しないパラメータの補間
        //   - 自由度を持つパラメータからx,y,w,hを一意に決定
        //   - gradationからcolorを作成

        // todo:
        // パラメータの数が多すぎる/少なすぎる場合にどうするかを考える(今は適当に処理
        // 値が不正(w,hがマイナスなど)の時にどこでチェックをするかを考える

        let mut r = Rect {
            canvas_id: value.canvas_id,
            shape_id : value.shape_id,
            ..Default::default()
        };

        r.z = value.z.or(r.z);
        r.theta = value.theta.or(r.theta);
        r.color = value.color.or(r.color);

        // update color by gradation
        if value.grad_begin.is_some() && value.grad_end.is_some() && value.grad_ratio.is_some() {
            r.color = Some(Color::from_gradation(&value.grad_begin.unwrap(), &value.grad_end.unwrap(), value.grad_ratio.unwrap()));
        }

        let xw = match (value.left, value.right, value.width, value.center_x) {
            (Some(l), Some(r), _, _) => Ok(( Some(l)     , Some(r-l)      )),
            (Some(l), _, Some(w), _) => Ok(( Some(l)     , Some(w)        )),
            (Some(l), _, _, Some(c)) => Ok(( Some(l)     , Some((c-l)*2.) )),
            (_, Some(r), Some(w), _) => Ok(( Some(r-w)   , Some(w)        )),
            (_, Some(r), _, Some(c)) => Ok(( Some(2.*c-r), Some((r-c)*2.) )),
            (_, _, Some(w), Some(c)) => Ok(( Some(c-w/2.), Some(w)        )),
            _ => Err("Unable to determine x and w")
        }?;
        let yh = match (value.top, value.bottom, value.height, value.center_y) {
            (Some(t), Some(b), _, _) => Ok(( Some(t)     , Some(b-t)      )),
            (Some(t), _, Some(h), _) => Ok(( Some(t)     , Some(h)        )),
            (Some(t), _, _, Some(c)) => Ok(( Some(t)     , Some((c-t)*2.) )),
            (_, Some(b), Some(h), _) => Ok(( Some(b-h)   , Some(h)        )),
            (_, Some(b), _, Some(c)) => Ok(( Some(2.*c-b), Some((b-c)*2.) )),
            (_, _, Some(h), Some(c)) => Ok(( Some(c-h/2.), Some(h)        )),
            _ => Err("Unable to determine y and h")
        }?;

        r.x = xw.0.or(r.x);
        r.y = yh.0.or(r.y);
        r.w = xw.1.or(r.w);
        r.h = yh.1.or(r.h);

        Ok(r)
    }
}

impl<'a> Visualizable<'a> for MultiParametricRect<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "canvasID"  => { self.canvas_id  = word;                            Ok(()) },
            "shapeID"   => { self.shape_id   = word;                            Ok(()) },

            "color"     => { self.color      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradBegin" => { self.grad_begin = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradEnd"   => { self.grad_end   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradRatio" => { self.grad_ratio = Some(word.parse_based_on(ctx)?); Ok(()) },

            "left"      => { self.left       = Some(word.parse_based_on(ctx)?); Ok(()) },
            "right"     => { self.right      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "width"     => { self.width      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "centerX"   => { self.center_x   = Some(word.parse_based_on(ctx)?); Ok(()) },

            "top"       => { self.top        = Some(word.parse_based_on(ctx)?); Ok(()) },
            "height"    => { self.height     = Some(word.parse_based_on(ctx)?); Ok(()) },
            "centerY"   => { self.center_y   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "bottom"    => { self.bottom     = Some(word.parse_based_on(ctx)?); Ok(()) },

            "z"         => { self.z          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "theta"     => { self.theta      = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'MultiParametricRect': {}. please check settings", param_name).into())
        }
    }
}

impl<'a> VisualizableFrom<'a, MultiParametricRect<'a>> for Rect<'a> {
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            canvas_id: self.canvas_id,
            shape_id: self.shape_id,
            color: if self.color == other.color { None } else { self.color.clone() },
            x:     if self.x     == other.x     { None } else { self.x },
            y:     if self.y     == other.y     { None } else { self.y },
            w:     if self.w     == other.w     { None } else { self.w },
            h:     if self.h     == other.h     { None } else { self.h },
            z:     if self.z     == other.z     { None } else { self.z },
            theta: if self.theta == other.theta { None } else { self.theta },
        }
    }
}
