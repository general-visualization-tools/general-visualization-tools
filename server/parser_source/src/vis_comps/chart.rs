use serde::{ Serialize, Serializer };
use std::error::Error;
use std::collections::HashMap;
use crate::context::Context;
use super::traits::{ ParsableBasedOnCtx, Visualizable };
use super::common_types::{ number::Number, point::Point, color::{ Color, Palette } };

/*
{
  'groupID': [
    { "name": "", "data": [[x,y], [x,y], ], "color": "#000000" }
    { "name": "", "data": [y,y,y,y ], "color": "#000000" }
    ...
  ]
}
*/

#[derive(Debug, Default)]
pub struct ChartCreator<'a> { chart: Chart<'a> }

#[derive(Debug, Default, Clone, Serialize)]
pub struct Chart<'a> {
    #[serde(flatten)]
    line_id_to_line: HashMap<&'a str, Line<'a>>
}

// xが0-indexedで連番ならyだけのデータにし、それ以外なら[[$x, $y]...]の形式でserialize
fn serialize_data<S>(data: &[Point], s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let is_consecutive = data.iter().fold((true, -1), |(f, prev_x), p| { (f && prev_x+1 == p.x as i32, p.x as i32) }).0;
    if is_consecutive { data.iter().map(|p| p.y).collect::<Vec<_>>().serialize(s) }
    else { data.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>().serialize(s) }
}

#[derive(Debug, Clone, Serialize)]
struct Line<'a> {
    #[serde(rename="name")] // highcharts側のlabel名を使用
    line_id: &'a str,

    #[serde(flatten)]
    color: Color,

    #[serde(serialize_with="serialize_data")]
    data: Vec<Point>,
}

#[derive(Debug)]
pub struct LineDatum<'a> {
    pub group_id: &'a str,
    line_id: &'a str,
    color: Option<Color>,
    x: Option<Number>,
    y: Option<Number>,
}

impl<'a> Default for Line<'a> {
    fn default() -> Self {
        Self {
            line_id: "line0",
            color: Palette::green(),
            data: Vec::new()
        }
    }
}

impl<'a> Default for LineDatum<'a> {
    fn default() -> Self {
        Self {
            group_id: "group0",
            line_id: "line0",
            color: None,
            x: None,
            y: None,
        }
    }
}

impl<'a> Visualizable<'a> for LineDatum<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "groupID" => { self.group_id  = word;                            Ok(()) },
            "lineID"  => { self.line_id   = word;                            Ok(()) },
            "color"   => { self.color     = Some(word.parse_based_on(ctx)?); Ok(()) },
            "x"       => { self.x         = Some(word.parse_based_on(ctx)?); Ok(()) },
            "y"       => { self.y         = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'DatumForChart': {}. please check settings", param_name).into())
        }
    }
}

impl<'a> ChartCreator<'a> {
    pub fn add_line_datum(&mut self, datum: LineDatum<'a>) {
        let mut line = self.chart.line_id_to_line.entry(datum.line_id).or_insert_with(Line::default);
        if let Some(color) = datum.color { line.color = color; }
        if let (Some(x), Some(y)) = (datum.x, datum.y) {
            line.data.push(Point { x, y });
        }
    }
    pub fn create_chart(&mut self) -> Chart<'a> {
        for line in self.chart.line_id_to_line.values_mut() {
            line.data.sort_by(|p, other| p.x.partial_cmp(&other.x).unwrap());
        }
        self.chart.clone()
    }
}
