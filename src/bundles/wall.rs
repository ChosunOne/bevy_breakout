use bevy::prelude::*;

use crate::{
    components::Collider,
    constants::{
        color::WALL_COLOR,
        stage::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS},
    },
};

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    pub fn new(location: WallLocation) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(LEFT_WALL, 0.0),
            Self::Right => Vec2::new(RIGHT_WALL, 0.0),
            Self::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            Self::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            Self::Left | Self::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            Self::Bottom | Self::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}
