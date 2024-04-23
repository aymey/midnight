use std::cmp::Ordering;
use crate::{Frame, object::*};
use crate::ease;

#[derive(Debug, Copy, Clone)]
pub struct Keyframe<'a> {
    pub id: usize,

    pub begin: Frame,
    pub end: Frame,
    pub(crate) action: Action<'a>,

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
        self.action.perform(object);
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ActionKind {
    #[default]
    None,
    FadeIn,
    FadeOut,
    Morph,
    Teleport,
}

/// attached to [Keyframe]
/// shows the object at the end of the keyframes duration
/// and explains how it transforms to get to its new state
#[derive(Debug, Clone, Copy)]
pub struct Action<'a> {
    object: &'a Object<'a>,
    kind: ActionKind,
    medium: ease::EaseKind,
}

impl<'a> Action<'a> {
    pub fn new(object: &'a Object) -> Self {
        Self {
            object,
            kind: Default::default(),
            medium: Default::default()
        }
    }

    pub fn perform(&self, original: &mut Object) {

    }
}
