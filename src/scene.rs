use crate::{consts::*, system::*};

use bevy_ecs::prelude::*;
use nannou::geom::Rect;
use nannou::lyon::math as lyon;

#[derive(Resource)]
pub struct Bounds(Rect);

impl Bounds {
    pub fn new(rect: Rect) -> Self {
        Self(Rect::from_x_y_w_h(
                rect.x() / ZOOM, rect.y() / ZOOM,
                rect.w() / ZOOM, rect.h() / ZOOM
            )
        )
    }
}

pub struct Scene {
    world: World,
    updater: Schedule,
    drawer: Schedule,
    event_time: f32,
    clock_time: f32,
    creation_count: u32,
    transform: lyon::Transform
}

impl Scene {
    pub fn new(window: Rect) -> Self {
        let mut world = World::new();
        let transform = lyon::Transform::identity()
            .then_scale(ZOOM, ZOOM);
        let bounds = Bounds::new(window);
        world.insert_resource(Time::default());
        world.insert_resource(bounds);
        world.insert_resource(transform);
    }
}
