use serde::{ Serialize };
use std::error::Error;
use core::slice::Iter;
use std::collections::HashMap;
use crate::GraphicPartsSetting;
use crate::context::Context;
use super::graphic_elems::{ camera::Camera, circle::Circle, rect::Rect, path::Path };
use super::graphic::{ Graphic, GraphicCreator };
use super::chart::{ LineDatum, Chart, ChartCreator };
use super::traits::{ Visualizable, VisualizableFrom };
use super::graphic_elems::elems::ElementTrait;

#[derive(Debug, Default, Serialize)]
struct VisComps<'a> {
    #[serde(rename="groupID")]
    group_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    graphic: Option<Graphic<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chart: Option<Chart<'a>>,
}

#[derive(Debug, Default)]
pub struct VisCompsCreator<'a> {
    group_id_to_graphic_creator: HashMap<&'a str, GraphicCreator<'a>>,
    group_id_to_chart_creator: HashMap<&'a str, ChartCreator<'a>>,
}

impl<'a> VisCompsCreator<'a> {
    pub fn add_line_datum_from(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let line_datum = LineDatum::from_words_and_setting(words_iter, setting, ctx)?;
        self.group_id_to_chart_creator.entry(line_datum.group_id).or_insert_with(ChartCreator::default)
            .add_line_datum(line_datum);
        Ok(())
    }

    fn add_elem<T: ElementTrait<'a>>(&mut self, elem: T, group_id: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>> {
        self.group_id_to_graphic_creator.entry(group_id).or_insert_with(GraphicCreator::default).add(elem, ctx);
        Ok(())
    }
    pub fn add_path_from(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let elem = Path::from_words_and_setting(words_iter, setting, ctx)?;
        let group_id = elem.group_id;
        self.add_elem(elem, group_id, ctx)
    }
    pub fn add_rect_from(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let elem = Rect::from_words_and_setting(words_iter, setting, ctx)?;
        let group_id = elem.group_id;
        self.add_elem(elem, group_id, ctx)
    }
    pub fn add_circle_from(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let elem = Circle::from_words_and_setting(words_iter, setting, ctx)?;
        let group_id = elem.group_id;
        self.add_elem(elem, group_id, ctx)
    }
    pub fn add_camera_from(&mut self, words_iter: &mut Iter<&'a str>, setting: &'a GraphicPartsSetting, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let elem = Camera::from_words_and_setting(words_iter, setting, ctx)?;
        let group_id = elem.group_id;
        self.add_elem(elem, group_id, ctx)
    }

    pub fn create_json_string(&'a mut self) -> Result<String, serde_json::Error> {
        let group_id_to_vis_comps = self.group_id_to_graphic_creator.iter_mut().fold(
            self.group_id_to_chart_creator.iter_mut().fold( HashMap::new(), |mut map, (&group_id, creator)| {
                map.insert(group_id, VisComps { group_id, graphic: None, chart: Some(creator.create_chart()) });
                map
            }),
            |mut map, (&group_id, creator)| {
                map.entry(group_id).or_insert_with(|| VisComps { group_id, ..VisComps::default() })
                    .graphic = Some(creator.create_graphic());
                map
            }
        );

        serde_json::to_string(&group_id_to_vis_comps.values().collect::<Vec<_>>())
    }

}
