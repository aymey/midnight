use nannou::geom::Vec2;
use crate::{Angle, Frame, animation::*};
use std::cmp::Ordering::*;

#[derive(Default, Debug, Clone, Copy)]
pub enum ObjectKind {
    #[default]
    Square,
    Circle,
    Triangle
}

#[derive(Default, Debug)]
pub struct Object<'a> {
    kind: ObjectKind,
    position: Vec2,
    size: Vec2,
    rotation: Angle,
    keyframes: Vec<&'a Keyframe<'a>>
}

impl<'a> Object<'a> {
    pub fn update(&mut self, frame: Frame) {

    }

    pub fn keyframe(&self, frame: Frame) -> Option<&Keyframe> {
        let mut left = 0;
        let mut right = self.keyframes.len() - 1;

        while left < right {
            let m = (left + right) / 2;
            let start = self.keyframes[m].begin;

            match start.cmp(&frame) {
                Less    => left = m,
                Greater => right = m,
                Equal   => return Some(self.keyframes[m])
            }
        }

        None
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
