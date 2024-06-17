use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct ScoreboardUi;
