use bevy::prelude::*;
use bevy_breakout::{
    constants::color::BACKGROUND_COLOR,
    events::{play_collision_sound, CollisionEvent},
    resources::Score,
    setup,
    systems::{apply_velocity, check_for_collisions, move_paddle, update_scoreboard},
};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
                play_collision_sound,
            )
                .chain(),
        )
        .add_systems(Update, update_scoreboard)
        .run();
}
