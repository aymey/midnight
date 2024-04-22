use std::cmp::Ordering;

use nannou::geom::Vec2;

type Angle = f32;
type Frame = u32;

/// global state
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
            object.update();
        }
    }

    pub fn display(&self) {

    }
}

#[derive(Default)]
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
    pub fn update(&mut self) {
        for keyframe in self.keyframes.iter() {
            keyframe.
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

#[derive(Default, Debug, Clone, Copy)]
pub struct Action {

}

impl Action {
    fn perform(&self) {

    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Keyframe<'a> {
    id: usize,

    begin: Frame,
    end: Frame,
    action: Option<Action>,

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

impl Keyframe<'_> {
    fn action(&mut self) {
        if let Some(action) = self.action {
            action.perform();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::*;

    #[test]
    fn object_add_keyframe() {
        let mut object = Object::default();
        let mut first = Keyframe {
            id: 0,
            begin: 2,
            end: 10,
            action: None,
            _marker: PhantomData
        };
        let mut second = Keyframe {
            id: 0,
            begin: 30,
            end: 37,
            action: None,
            _marker: PhantomData
        };
        let fc = first.clone(); let sc = second.clone();
        object.add_keyframe(&mut second);
        object.add_keyframe(&mut first);

        let keyframes: Vec<Keyframe> = object.keyframes.iter().map(|&k| *k).collect();
        assert_eq!(vec![fc, sc], keyframes);
    }
}
