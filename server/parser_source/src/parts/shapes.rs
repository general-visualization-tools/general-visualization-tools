use serde::{Serialize};
use std::collections::HashMap;

use crate::context::Context;
use crate::parts::rect::Rect;
use crate::parts::circle::Circle;
use crate::parts::basic_types::Number;
use crate::parts::traits::VisualizableFrom;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum Shape<'a> { Rect(Rect<'a>), Circle(Circle<'a>) }

#[derive(Debug)]
struct Patch<'a> { time: Number, shape: Shape<'a> }

#[derive(Debug, Serialize)]
struct Diff<'a> { next: Shape<'a>, prev: Shape<'a> }

#[derive(Debug, Default, Serialize)]
struct Transition<'a> { time: Number, diffs: Vec<Diff<'a>> }

#[derive(Debug, Default, Clone, Serialize)]
struct Frame<'a> {
    time: Number,
    #[serde(rename="shapes")]
    shape_id_to_shape: HashMap<&'a str, &'a Shape<'a>>
}

#[derive(Debug, Default, Serialize)]
pub struct Canvas<'a> {
    #[serde(rename="initial")]
    initial_frame: Frame<'a>,
    #[serde(rename="final")]
    final_frame: Frame<'a>,
    transitions: Vec<Transition<'a>>,
}

#[derive(Debug, Default)]
pub struct Canvases<'a> { patches_each_canvas: HashMap<&'a str, Vec<Patch<'a>>> }

impl<'a> Shape<'a> {
    fn get_shape_id(&self) -> &'a str { match self { Shape::Rect(r) => r.shape_id, Shape::Circle(c) => c.shape_id } }
    fn get_diff_as_next_from(&self, prev_shape: &Shape<'a>) -> Diff<'a> {
        match (self, prev_shape) {
            (Shape::Rect(next_rect), Shape::Rect(prev_rect)) => {
                Diff {
                    next: Shape::Rect(next_rect.extract_diff_from(prev_rect)),
                    prev: Shape::Rect(prev_rect.extract_diff_from(next_rect)),
                }
            },
            (Shape::Circle(next_circle), Shape::Circle(prev_circle)) => {
                Diff {
                    next: Shape::Circle(next_circle.extract_diff_from(prev_circle)),
                    prev: Shape::Circle(prev_circle.extract_diff_from(next_circle)),
                }
            },
            (_,_) => unreachable!("shapes isn't same")
        }
    }
}

impl<'a> Canvases<'a> {
    pub fn add_rect(&mut self, rect: Rect<'a>, ctx: &Context) {
        self.patches_each_canvas.entry(rect.canvas_id).or_insert(Vec::new())
            .push(Patch { time: ctx.current_time, shape: Shape::Rect(rect) });
    }

    pub fn add_circle(&mut self, circle: Circle<'a>, ctx: &Context) {
        self.patches_each_canvas.entry(circle.canvas_id).or_insert(Vec::new())
            .push(Patch { time: ctx.current_time, shape: Shape::Circle(circle) });
    }

    pub fn create_map(&'a mut self) -> HashMap<&'a str, Canvas> {
        self.patches_each_canvas.iter_mut().fold(HashMap::new(), |mut result_map, (canvas_id, patches)| {
            patches.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

            let grouped_patches = patches.iter().fold(Vec::new(), |mut v: Vec<Vec<&Patch>>, patch| {
                if v.is_empty() || v.last().unwrap()[0].time != patch.time { v.push(Vec::new()); }
                v.last_mut().unwrap().push(patch);
                v
            });

            let (mut canvas, final_frame, _) = grouped_patches.iter().fold((Canvas::default(), Frame::default(), true), |(mut canvas, current_frame, is_first), patches| {
                let time = patches[0].time;
                let (next_frame, transition) = patches.iter().fold((Frame { time, ..current_frame }, Transition { time, diffs: Vec::default() }), |(mut frame, mut transition), patch| {
                    if let Some(diff) = frame.shape_id_to_shape.insert(patch.shape.get_shape_id(), &patch.shape)
                        .map(|prev_shape| patch.shape.get_diff_as_next_from(prev_shape)) {
                        transition.diffs.push(diff);
                    }
                    (frame, transition)
                });
                if is_first { canvas.initial_frame = next_frame.clone(); }
                else { canvas.transitions.push(transition); }
                (canvas, next_frame, false)
            });
            canvas.final_frame = final_frame;

            result_map.insert(canvas_id, canvas);

            result_map
        })
    }
}
