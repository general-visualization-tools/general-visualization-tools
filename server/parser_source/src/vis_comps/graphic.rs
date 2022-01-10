use serde::{ Serialize, Serializer, ser::SerializeStruct };
use std::collections::HashMap;
use crate::context::Context;
use super::common_types::number::Number;
use super::graphic_elems::deleter::ElemDeleter;
use super::unique_id_generator::UIDGen;
use super::graphic_elems::elems::{ Elem, ElementTrait };

#[derive(Debug)]
struct Patch<'a> { time: Number, elem: Elem<'a> }

#[derive(Debug, Serialize)]
#[serde(tag = "diffType")]
enum Diff<'a> { Create(Elem<'a>), Update(Elem<'a>), Delete(ElemDeleter) }

#[derive(Debug, Default, Serialize)]
struct Transition<'a> { time: Number, prev: Vec<Diff<'a>>, next: Vec<Diff<'a>> }

#[derive(Debug, Default, Clone)]
struct Frame<'a> {
    time: Number,
    elem_id_to_elem: HashMap<(&'a str, &'a str), &'a Elem<'a>>
}

#[derive(Debug, Default, Serialize)]
pub struct Graphic<'a> {
    #[serde(rename="initial")]
    initial_frame: Frame<'a>,
    #[serde(rename="final")]
    final_frame: Frame<'a>,
    transitions: Vec<Transition<'a>>,
}

#[derive(Debug, Default)]
pub struct GraphicCreator<'a> {
    patches: Vec<Patch<'a>>,
}

impl<'a> GraphicCreator<'a> {
    pub fn add<T: ElementTrait<'a>>(&mut self, elem: T, ctx: &Context) {
        self.patches.push(Patch { time: ctx.current_time, elem: elem.convert_to_elem() });
    }
    pub fn create_graphic(&'a mut self) -> Graphic<'a> {
        let mut uid_gen = UIDGen::new();

        self.patches.sort_by(|p, other| p.time.partial_cmp(&other.time).unwrap());

        // give UID each elems via Vec<UID> to avoid "cannot borrow as mutable because it is also borrowed as immutable"
        self.patches.iter().fold((Vec::new(), HashMap::new()), |(mut v, mut map), p| {
            v.push(*map.entry(p.elem.get_elem_id()).or_insert_with(|| uid_gen.gen())); (v, map)
        }).0.into_iter().enumerate().map(|(i, uid)| { self.patches[i].elem.set_unique_id(uid) }).for_each(drop);

        let grouped_patches = self.patches.iter_mut().fold(Vec::new(), |mut v: Vec<Vec<&Patch>>, patch| {
            if v.is_empty() || v.last().unwrap()[0].time != patch.time { v.push(Vec::new()); }
            v.last_mut().unwrap().push(patch);
            v
        });

        let (mut graphic, final_frame, _) = grouped_patches.iter().fold((Graphic::default(), Frame::default(), true), |(mut graphic, current_frame, is_first), patches| {
            let time = patches[0].time;
            let (next_frame, transition) = patches.iter().fold((Frame { time, ..current_frame }, Transition { time, ..Transition::default() }), |(mut frame, mut transition), patch| {
                if let Some(prev_elem) = frame.elem_id_to_elem.insert(patch.elem.get_elem_id(), &patch.elem) {
                    transition.next.push(Diff::Update(patch.elem.extract_diff_from(prev_elem)));
                    transition.prev.push(Diff::Update(prev_elem.extract_diff_from(&patch.elem)));
                } else {
                    transition.prev.push(Diff::Delete(patch.elem.get_this_elem_deleter()));
                    transition.next.push(Diff::Create(patch.elem.clone()));
                }
                (frame, transition)
            });
            if is_first {
                graphic.initial_frame = next_frame.clone();
                graphic.transitions.push(Transition { time, ..Transition::default() });
            }
            else {
                graphic.transitions.last_mut().unwrap().next = transition.next;
                graphic.transitions.push(Transition { time, prev: transition.prev, ..Transition::default() });
            }
            (graphic, next_frame, false)
        });

        graphic.final_frame = final_frame;

        graphic
    }
}

impl<'a> Serialize for Frame<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer
    {
        let mut state = serializer.serialize_struct("Frame", 2)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("elems", &self.elem_id_to_elem.values().collect::<Vec<_>>())?;
        state.end()
    }
}