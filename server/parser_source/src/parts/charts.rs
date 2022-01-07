use serde::{Serialize};
use std::error::Error;
use core::slice::Iter;
use std::collections::HashMap;

use crate::setting::PartsSetting;
use crate::context::Context;
use super::color::{ Color, Palette };
use super::basic_types::Number;
use super::traits::{ ParsableBasedOnCtx, Visualizable };

/*
{
  'canvasID': [
    { "name": "", "data": [[x,y], [x,y], ], "color": "#000000" }
    { "name": "", "data": [y,y,y,y ], "color": "#000000" }
    ...
  ]
}
*/

#[derive(Default, Debug)]
pub struct Charts<'a> {
    charts: HashMap<(&'a str, &'a str), Chart<'a>>
}

#[derive(Serialize, Debug)]
pub struct Chart<'a> {
    #[serde(skip)]
    canvas_id: &'a str,
    #[serde(rename="name")]
    chart_id: &'a str,

    #[serde(flatten)]
    color: Color,

    data: Vec<(Number, Number)>,
}

struct DatumForChart<'a> {
    canvas_id: &'a str,
    chart_id: &'a str,
    color: Option<Color>,
    x: Option<Number>,
    y: Option<Number>,
}

impl<'a> Default for Chart<'a> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            chart_id: "chart0",
            color: Palette::green(),
            data: Vec::new()
        }
    }
}

impl<'a> Default for DatumForChart<'a> {
    fn default() -> Self {
        Self {
            canvas_id: "canvas0",
            chart_id: "chart0",
            color: None,
            x: None,
            y: None,
        }
    }
}

impl<'a> Visualizable<'a> for DatumForChart<'a> {
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        match param_name {
            "canvasID" => { self.canvas_id = word;                            Ok(()) },
            "chartID"  => { self.chart_id  = word;                            Ok(()) },
            "color"    => { self.color     = Some(word.parse_based_on(ctx)?); Ok(()) },
            "x"        => { self.x         = Some(word.parse_based_on(ctx)?); Ok(()) },
            "y"        => { self.y         = Some(word.parse_based_on(ctx)?); Ok(()) },
            _ => Err(format!("this field name is not exists in 'DatumForChart': {}. please check settings", param_name).into())
        }
    }
}

impl<'a> Charts<'a> {
    fn add_datum(&mut self, datum: &DatumForChart<'a>) {
        let key = (datum.canvas_id, datum.chart_id);
        self.charts.entry(key).or_insert_with(|| Chart {
            canvas_id: datum.canvas_id,
            chart_id: datum.chart_id,
            ..Default::default()
        });

        let chart = self.charts.get_mut(&key).unwrap();
        chart.merge_datum(datum);
    }
    pub fn add_datum_by_words_and_setting(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a PartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let mut datum = DatumForChart::default_by_setting(setting, ctx)?;

        for params_name in setting.input_params.iter() {
            let next_word = *words_iter.next().ok_or(format!("words iter don't have more words. required: {}", params_name))?;
            datum.set_by_param_name_and_word(params_name.as_str(), next_word, ctx)?;
        }

        self.add_datum(&datum);

        Ok(())
    }

    pub fn create_map(&'a self) -> HashMap<&'a str, Vec<&'a Chart<'a>>> {
        self.charts.values().fold(HashMap::new(), |mut result_map, chart| {
            result_map.entry(chart.canvas_id).or_insert_with(Vec::new).push(chart);
            result_map
        })
    }
}

impl<'a> Chart<'a> {
    fn merge_datum(&mut self, datum: &DatumForChart) {
        if let Some(color) = datum.color.clone() { self.color = color; }
        if datum.x.is_some() && datum.y.is_some() {
            self.data.push((datum.x.unwrap(), datum.y.unwrap()));
        }
    }
}
