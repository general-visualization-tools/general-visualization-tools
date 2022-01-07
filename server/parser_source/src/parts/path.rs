use core::slice::Iter;
use serde::{Serialize, Serializer};
use std::error::Error;
use std::convert::TryFrom;

use super::basic_types::Number;
use super::color::{ Color };
use super::traits::{ParsableBasedOnCtx, Visualizable, VisualizableFrom};
use crate::context::Context;
use crate::PartsSetting;

fn format_f32_data<S>(v: &Option<Vec<Number>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let b = v.as_ref().unwrap();
    let a = b.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    s.serialize_str(a.as_str())
}

#[derive(Serialize, Clone, Debug)]
pub struct Path<'a>  {
    #[serde(skip)]
    pub(in super) canvas_id: &'a str,
    #[serde(rename="shapeID")]
    pub(in super) shape_id: &'a str,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(serialize_with="format_f32_data", skip_serializing_if = "Option::is_none")]
    points: Option<Vec<Number>>,
}

#[derive(Debug)]
pub struct MultiParametricPath<'a> {
    canvas_id: &'a str,
    shape_id:  &'a str,

    color:      Option<Color>,
    grad_begin: Option<Color>,
    grad_end:   Option<Color>,
    grad_ratio: Option<Number>,

    n: Option<Number>,
    points: Vec<Number>,
}

impl Default for Path<'_> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            shape_id: "path0",

            color: Some(Color::default()),

            points: Some(Vec::new()),
        }
    }
}

impl Default for MultiParametricPath<'_> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            shape_id: "path0",

            color:      None,
            grad_begin: None,
            grad_end:   None,
            grad_ratio: None,

            n: None,
            points: Vec::new(),
        }
    }
}

impl<'a> TryFrom<MultiParametricPath<'a>> for Path<'a> {
    type Error = Box<dyn Error>;
    fn try_from(value: MultiParametricPath<'a>) -> Result<Self, Self::Error> {
        // 主に2つのことを行う
        //   - 存在しないパラメータの補間
        //   - gradationからcolorを作成

        let mut c = Path {
            canvas_id: value.canvas_id,
            shape_id : value.shape_id,
            points   : Some(value.points),
            ..Default::default()
        };

        c.color = value.color.or(c.color);
        if value.grad_begin.is_some() && value.grad_end.is_some() && value.grad_ratio.is_some() {
            c.color = Some(Color::from_gradation(&value.grad_begin.unwrap(), &value.grad_end.unwrap(), value.grad_ratio.unwrap()));
        }

        Ok(c)
    }
}

impl<'a> MultiParametricPath<'a> {
    fn set_points(&mut self, words_iter: &mut Iter<&'a str>, ctx: &Context) -> Result<(), Box<dyn Error>> {
        if self.n.is_none() { return Err("n is none when call set_points".into()); }

        for _ in 0..(self.n.unwrap() as usize)*2 {
            let next_word = *words_iter.next().ok_or("words iter don't have more words. required: points")?;
            let x = next_word.parse_based_on(ctx)?;
            self.points.push(x);
        }
        Ok(())
    }
}

impl<'a> Visualizable<'a> for MultiParametricPath<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "canvasID"  => { self.canvas_id  = word;                            Ok(()) },
            "shapeID"   => { self.shape_id   = word;                            Ok(()) },

            "color"     => { self.color      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradBegin" => { self.grad_begin = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradEnd"   => { self.grad_end   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradRatio" => { self.grad_ratio = Some(word.parse_based_on(ctx)?); Ok(()) },

            "n"         => { self.n          = Some(word.parse_based_on(ctx)?); Ok(()) },

            "points" => unreachable!("points should be deal with in a specialized way"),
            _ => Err(format!("this field name is not exists in 'MultiParametricRect': {}. please check settings", param_name).into())
        }
    }

    fn default_by_setting(setting: &'a PartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        let mut parts = Self::default();
        for (param_name, word) in &setting.default_values {
            if param_name == "points" {
                let words = word.split_whitespace().collect::<Vec<_>>();
                let mut words_iter = words.iter();
                parts.set_points(&mut words_iter, ctx)?;
            }
            else { parts.set_by_param_name_and_word(param_name.as_str(), word.as_str(), ctx)?; }
        }
        Ok(parts)
    }

    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a PartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        let mut parts = Self::default_by_setting(setting, ctx)?;
        for param_name in setting.input_params.iter() {
            if param_name == "points" { parts.set_points(words_iter, ctx)?; }
            else {
                let next_word = *words_iter.next().ok_or(format!("words iter don't have more words. required: {}", param_name))?;
                parts.set_by_param_name_and_word(param_name.as_str(), next_word, ctx)?;
            }
        }
        Ok(parts)
    }
}

impl<'a> VisualizableFrom<'a, MultiParametricPath<'a>> for Path<'a> {
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            canvas_id: self.canvas_id,
            shape_id: self.shape_id,
            color: if self.color == other.color { None } else { self.color.clone() },
            points: if self.points == other.points { None } else { self.points.clone() },
        }
    }
}
