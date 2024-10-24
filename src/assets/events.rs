use bevy::prelude::*;
use bevy_trickfilm::{animation::event::EventTarget, prelude::*};
use bevy_trickfilm_derive::AnimationEvent;

#[derive(Debug, Clone, Event, Reflect, AnimationEvent)]
pub struct SpawnHitboxEvent {
    #[reflect(skip_serializing)]
    #[target]
    pub target: EventTarget,
    pub msg: String,
}

pub struct AssetEventsPlugin;

impl Plugin for AssetEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_animation_event::<SpawnHitboxEvent>();
    }
}
