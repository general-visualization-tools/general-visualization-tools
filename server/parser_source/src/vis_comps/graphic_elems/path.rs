use serde::{Serialize, Serializer};
use std::convert::TryFrom;
use std::error::Error;
use core::slice::Iter;
use crate::GraphicPartsSetting;
use crate::context::Context;
use super::super::unique_id_generator::UID;
use super::super::common_types::{ number::Number, point::Point, color::Color };
use super::super::traits::{ ParsableBasedOnCtx, Visualizable, VisualizableFrom };
use super::elems::{ Elem, ElementTrait };

// svgで使いやすいように(x,y)...の空白区切りの文字列にする
fn serialize_points<S>(v: &Option<Vec<Point>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let v = v.as_ref().unwrap();
    let str_path = v.iter().map(|p| format!("{} {}", p.x, p.y)).collect::<Vec<_>>().join(" ");
    s.serialize_str(str_path.as_str())
}

#[derive(Serialize, Clone, Debug)]
pub struct Path<'a>  {
    #[serde(flatten)]
    pub (in super) unique_id: UID,
    #[serde(skip)]
    pub group_id: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub (in super) name: &'a str,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<Number>,

    #[serde(serialize_with="serialize_points", skip_serializing_if = "Option::is_none")]
    points: Option<Vec<Point>>,
}

#[derive(Debug)]
pub struct MultiParametricPath<'a> {
    unique_id: UID,
    group_id:  &'a str,
    name:      &'a str,

    color:      Option<Color>,
    grad_begin: Option<Color>,
    grad_end:   Option<Color>,
    grad_ratio: Option<Number>,

    z: Option<Number>,
    n: Option<Number>,
    points: Vec<Point>,
}

impl Default for Path<'_> {
    fn default() -> Self {
        Self {
            unique_id: UID::unset(),
            group_id:  "group0",
            name:      "path0",

            color: Some(Color::default()),

            z: Some(0.),

            points: Some(Vec::new()),
        }
    }
}

impl Default for MultiParametricPath<'_> {
    fn default() -> Self {
        Self {
            unique_id: UID::unset(),
            group_id: "group0",
            name:  "path0",

            color:      None,
            grad_begin: None,
            grad_end:   None,
            grad_ratio: None,

            z: None,
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

        let mut p = Path {
            unique_id: value.unique_id,
            group_id:  value.group_id,
            name:      value.name,
            points: Some(value.points),
            ..Default::default()
        };

        p.z = value.z.or(p.z);
        p.color = value.color.or(p.color);
        if value.grad_begin.is_some() && value.grad_end.is_some() && value.grad_ratio.is_some() {
            p.color = Some(Color::from_gradation(&value.grad_begin.unwrap(), &value.grad_end.unwrap(), value.grad_ratio.unwrap()));
        }

        Ok(p)
    }
}

impl<'a> MultiParametricPath<'a> {
    fn set_points(&mut self, words_iter: &mut Iter<&'a str>, ctx: &Context) -> Result<(), Box<dyn Error>> {
        if self.n.is_none() { return Err("n is none when call set_points".into()); }

        for _ in 0..self.n.unwrap() as usize {
            let next_word = *words_iter.next().ok_or("words iter don't have more words. required: points")?;
            let x = next_word.parse_based_on(ctx)?;
            let next_word = *words_iter.next().ok_or("words iter don't have more words. required: points")?;
            let y = next_word.parse_based_on(ctx)?;
            self.points.push(Point { x, y });
        }
        Ok(())
    }
}

impl<'a> Visualizable<'a> for MultiParametricPath<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "groupID"   => { self.group_id = word; Ok(()) },
            "name"      => { self.name     = word; Ok(()) },

            "color"     => { self.color      = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradBegin" => { self.grad_begin = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradEnd"   => { self.grad_end   = Some(word.parse_based_on(ctx)?); Ok(()) },
            "gradRatio" => { self.grad_ratio = Some(word.parse_based_on(ctx)?); Ok(()) },

            "z"         => { self.z          = Some(word.parse_based_on(ctx)?); Ok(()) },

            "n"         => { self.n          = Some(word.parse_based_on(ctx)?); Ok(()) },
            "points" => unreachable!("points should be deal with in a specialized way"),
            _ => Err(format!("this field name is not exists in 'MultiParametricPath': {}. please check settings", param_name).into())
        }
    }

    fn default_by_setting(setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
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

    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
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

// todo: default実装のみなのでimplを書かなくてもいいようにできるかどうか確かめる
impl<'a> VisualizableFrom<'a, MultiParametricPath<'a>> for Path<'a> {}

impl<'a> ElementTrait<'a> for Path<'a> {
    fn convert_to_elem(self) -> Elem<'a> { Elem::Path(self) }
    fn extract_diff_from(&self, other: &Self) -> Self {
        Self {
            unique_id: self.unique_id,
            group_id: if self.group_id == other.group_id { "" } else { self.group_id },
            name:     if self.name     == other.name     { "" } else { self.name },
            color:    if self.color == other.color { None } else { self.color.clone() },
            z:        if self.z     == other.z     { None } else { self.z },
            points: match (&self.points, &other.points) {
                (Some(sp), Some(op)) => if sp == op { None } else { Some(sp.clone()) },
                (Some(sp), None) => Some(sp.clone()),
                _ => None,
            }
        }
    }
}
