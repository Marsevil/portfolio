use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Aabb {
    pub aabb: Aabb3d,
}
