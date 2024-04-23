use crate::{Frame, object::*};

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

        self.update();
    }

    fn update(&mut self) {
        // for object in self.objects.iter_mut() {
        //     // object.update(self.frame);
        //     let keyframe = object.keyframe(self.frame);
        //     if let Some(keyframe) = keyframe {
        //         // keyframe.action.perform(object);
        //         keyframe.action
        //     }
        // }
    }

    pub fn display(&self) {
        for object in self.objects.iter() {
            println!("{object:?}")
        }
    }
}
