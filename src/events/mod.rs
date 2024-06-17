use bevy::prelude::*;

use crate::resources::CollisionSound;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
