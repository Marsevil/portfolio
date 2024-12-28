use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct UserData {
    pub selected_ent: Option<Entity>,
}

#[derive(Debug, Event)]
pub struct CastRayEvent {
    pub ray: Ray3d,
}

#[derive(Debug, Event)]
pub enum UnlockEntity {
    MouseExited,
    ClickReleased,
    MovingPlaneIntersectionNotFound,
}
