mod animations;
mod attack;
mod stagger;

pub use animations::{dude_dashing_sprites, dude_state_animation, DudeAnimations};
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
    Dashing,
    Parrying(ParryState),
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum ParryState {
    #[default]
    Start,
    Fail,
    Success,
}
