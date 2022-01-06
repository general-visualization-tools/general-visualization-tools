use serde::{ Serialize, Serializer, ser::SerializeStruct };
use std::collections::HashMap;

use crate::context::Context;
use super::path::Path;
use super::rect::Rect;
use super::circle::Circle;
use super::basic_types::Number;
use super::traits::VisualizableFrom;

#[derive(Debug, Clone, Serialize)]
struct ForDeletion<'a> {
    #[serde(rename="shapeID")]
    shape_id: &'a str
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum Shape<'a> { Rect(Rect<'a>), Circle(Circle<'a>), Path(Path<'a>), ForDeletion(ForDeletion<'a>) }

#[derive(Debug)]
struct Patch<'a> { time: Number, shape: Shape<'a> }

#[derive(Debug, Serialize)]
#[serde(tag = "diffType")]
enum Diff<'a> { Create(Shape<'a>), Update(Shape<'a>), Delete(Shape<'a>) }

#[derive(Debug, Default, Serialize)]
struct Transition<'a> { time: Number, prev: Vec<Diff<'a>>, next: Vec<Diff<'a>> }

#[derive(Debug, Default, Clone)]
struct Frame<'a> {
    time: Number,
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
    fn get_shape_id(&self) -> &'a str {
        match self {
            Shape::Rect(r) => r.shape_id,
            Shape::Circle(c) => c.shape_id,
            Shape::Path(p) => p.shape_id,
            Shape::ForDeletion(d) => d.shape_id,
        }
    }
    fn get_shape_for_deletion(&self) -> Shape { Shape::ForDeletion(ForDeletion { shape_id: self.get_shape_id() }) }
    fn extract_diff_from(&self, other: &Self) -> Self {
        match (self, other) {
            (Shape::Rect(rect), Shape::Rect(other)) => {
                Shape::Rect(rect.extract_diff_from(other))
            },
            (Shape::Circle(circle), Shape::Circle(other)) => {
                Shape::Circle(circle.extract_diff_from(other))
            },
            (Shape::Path(path), Shape::Path(other)) => {
                Shape::Path(path.extract_diff_from(other))
            }
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

    pub fn add_path(&mut self, path: Path<'a>, ctx: &Context) {
        self.patches_each_canvas.entry(path.canvas_id).or_insert(Vec::new())
            .push( Patch { time: ctx.current_time, shape: Shape::Path(path) });
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
                let (next_frame, transition) = patches.iter().fold((Frame { time, ..current_frame }, Transition { time, prev: Vec::default(), next: Vec::default() }), |(mut frame, mut transition), patch| {
                    if let Some(prev_shape) = frame.shape_id_to_shape.insert(patch.shape.get_shape_id(), &patch.shape) {
                        transition.next.push(Diff::Update(patch.shape.extract_diff_from(prev_shape)));
                        transition.prev.push(Diff::Update(prev_shape.extract_diff_from(&patch.shape)));
                    } else {
                        transition.prev.push(Diff::Delete(patch.shape.get_shape_for_deletion()));
                        transition.next.push(Diff::Create(patch.shape.clone()));
                    }
                    (frame, transition)
                });
                if is_first {
                    canvas.initial_frame = next_frame.clone();
                    canvas.transitions.push(Transition { time, ..Transition::default() });
                }
                else {
                    canvas.transitions.last_mut().unwrap().next = transition.next;
                    canvas.transitions.push(Transition { time, prev: transition.prev, ..Transition::default() });
                }
                (canvas, next_frame, false)
            });

            canvas.final_frame = final_frame;

            result_map.insert(canvas_id, canvas);

            result_map
        })
    }
}

impl<'a> Serialize for Frame<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer
    {
        let mut state = serializer.serialize_struct("Frame", 1)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("shapes", &self.shape_id_to_shape.values().collect::<Vec<_>>())?;
        state.end()
    }
}