use std::cmp::Ordering;

use nannou::geom::Vec2;

type Angle = f32;
type Frame = u32;

/// global state
#[derive(Default)]
pub struct Scene {
    frame: Frame,
    objects: Vec<Object>,
}

impl Scene {
    /// advances one frame ahead
    pub fn advance(&mut self) {
        self.frame += 1;

        for object in self.objects.iter_mut() {
            object.update();
        }
    }
    pub fn step(&mut self) { self.advance() }

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
pub struct Object {
    kind: ObjectKind,
    position: Vec2,
    size: Vec2,
    rotation: Angle,
    keyframes: Vec<Keyframe>
}

impl Object {
    pub fn update(&mut self) {
        for keyframe in self.keyframes.iter() {

        }
    }

    pub fn add_keyframe(&mut self, mut keyframe: Keyframe) {
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

#[derive(Default)]
pub struct Action {

}

#[derive(Default)]
pub struct Keyframe {
    id: usize,

    begin: Frame,
    duration: Frame, // duration or end?
    action: Action
}

impl Ord for Keyframe {
    fn cmp(&self, other: &Self) -> Ordering {
        self.begin.cmp(&other.begin)
    }
}

impl PartialOrd for Keyframe {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Keyframe {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin
    }
}

impl Eq for Keyframe { }

impl Keyframe {
    // fn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a {

    }
}
