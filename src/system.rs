use bevy_ecs::prelude::*;
use std::time;

#[derive(Resource, Default)]
pub struct Time {
    pub seconds: f32,
    pub count: u64,
    pub begin: Option<time::Instant>,
}
