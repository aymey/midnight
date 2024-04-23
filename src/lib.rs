use std::cmp::Ordering;
use nannou::geom::Vec2;
use std::rc::Rc;

mod ease;

type Angle = f32;
type Frame = u32;

/// global state
// TODO: maybe use nannou time module aswell
#[derive(Default)]
pub struct Scene<'a> {
    frame: Frame,
    objects: Vec<&'a mut Object<'a>>,
}

impl Scene<'_> {
    /// advances one frame ahead
    pub fn advance(&mut self) {
        self.frame += 1;

        for object in self.objects.iter_mut() {
            object.update(self.frame);
        }
    }

    pub fn display(&self) {

    }
}

#[derive(Default, Clone, Copy)]
pub enum ObjectKind {
    #[default]
    Square,
    Circle,
    Triangle
}

#[derive(Default)]
pub struct Object<'a> {
    kind: ObjectKind,
    position: Vec2,
    size: Vec2,
    rotation: Angle,
    keyframes: Vec<&'a Keyframe<'a>>
}

impl<'a> Object<'a> {
    pub fn update(&mut self, frame: Frame) {
        let mut left = 0;
        let mut right = self.keyframes.len() - 1;

        while left < right {
            let m = (left + right) / 2;

            if self.keyframes[m].begin < frame {
                left = m;
            } else if self.keyframes[m].begin > frame {
                right = m;
            }
        }
    }

    pub fn add_keyframe(&mut self, keyframe: &'a mut Keyframe) {
        keyframe.id = self.keyframes.len();
        self.keyframes.push(keyframe);
        self.keyframes.sort();
    }

    // TODO: better data structure for faster removal and lookups, prob just use a singly linked list
    // TODO: better error handling;
    pub fn remove_keyframe(&mut self, id: usize) {
        let position = self.keyframes.iter()
            .position(|k| k.id == id);
        if let Some(pos) = position {
            self.keyframes.remove(pos);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Keyframe<'a> {
    id: usize,

    begin: Frame,
    end: Frame,
    action: Action<'a>,

    _marker: std::marker::PhantomData<&'a ()>
}

impl Ord for Keyframe<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.begin.cmp(&other.begin)
    }
}

impl PartialOrd for Keyframe<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Keyframe<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin
    }
}

impl Eq for Keyframe<'_> { }

impl<'a> Keyframe<'a> {
    fn new(object: &'a Object) -> Self {
        Self {
            id: Default::default(),
            begin: Default::default(),
            end: Default::default(),
            action: Action::new(object),
            _marker: std::marker::PhantomData
        }
    }

    /// performs the action on the origin [Object]
    fn perform(&self, object: &mut Object) {
        self.action.perform();
    }
}

/// attached to [Keyframe]
/// shows the object at the end of the keyframes duration
/// and explains how it transforms to get to its new state
#[derive(Clone, Copy)]
pub struct Action<'a> {
    object: &'a Object<'a>,
    medium: ease::EaseKind
}

impl<'a> Action<'a> {
    pub fn new(object: &'a Object) -> Self {
        Self {
            object,
            medium: Default::default()
        }
    }

    pub fn perform(&self) {

    }
}


#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::*;
}
