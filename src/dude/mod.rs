mod animations;
mod attack;
mod stagger;

pub use animations::{
    dude_dashing_sprites, dude_state_animation_enemy, dude_state_animation_player, EnemyAnimations,
    PlayerAnimations,
};
pub use attack::{Attack, AttackForm};
pub use stagger::{Stagger, StaggerState};

use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((stagger::StaggerPlugin,));
    }
}

// TODO: Replace this with separate states for player and enemy?
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DudeState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
    Staggering,
    Stalking,
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
