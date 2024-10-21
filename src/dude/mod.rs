mod animations;
mod attack;
mod stagger;

pub use animations::{dude_state_animation, dude_state_hitbox_start_frame, DudeAnimations};
pub use attack::{Attack, AttackForm};
pub use stagger::{Stagger, StaggerState};

use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((stagger::StaggerPlugin,));
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DudeState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
    Staggering,
    Parrying,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ParryState {
    #[default]
    Start,
    Fail,
    Success,
    Recover,
}

#[derive(Default, Component)]
pub struct PreviousAttackFrame(pub usize);
