mod attack;
mod stagger;

pub use attack::{Attack, AttackForm};
pub use stagger::{Stagger, StaggerState};

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{assets::DudeAnimations, GameAssets};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DudeState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
    Staggering,
}

pub fn dude_state_animation(
    state: DudeState,
    attack: Attack,
    assets: &Res<GameAssets>,
) -> (Handle<AnimationClip2D>, bool) {
    match state {
        DudeState::Idling => (
            assets.dude_animations[DudeAnimations::Idle.index()].clone(),
            true,
        ),
        DudeState::Running => (
            assets.dude_animations[DudeAnimations::Run.index()].clone(),
            true,
        ),
        DudeState::Attacking => match attack {
            Attack::Light1 => (
                assets.dude_animations[DudeAnimations::Punch1.index()].clone(),
                false,
            ),
            Attack::Light2 => (
                assets.dude_animations[DudeAnimations::Punch2.index()].clone(),
                false,
            ),
            Attack::Light3 => (
                assets.dude_animations[DudeAnimations::Punch3.index()].clone(),
                false,
            ),
            Attack::Heavy1 => (
                assets.dude_animations[DudeAnimations::Kick1.index()].clone(),
                false,
            ),
            Attack::Heavy2 => (
                assets.dude_animations[DudeAnimations::Kick2.index()].clone(),
                false,
            ),
            Attack::Heavy3 => (
                assets.dude_animations[DudeAnimations::Kick3.index()].clone(),
                false,
            ),
        },
        DudeState::Recovering => match attack {
            Attack::Light1 => (
                assets.dude_animations[DudeAnimations::Punch1Recover.index()].clone(),
                false,
            ),
            Attack::Light2 => (
                assets.dude_animations[DudeAnimations::Punch2Recover.index()].clone(),
                false,
            ),
            Attack::Light3 => (
                assets.dude_animations[DudeAnimations::Punch3Recover.index()].clone(),
                false,
            ),
            Attack::Heavy1 => (
                assets.dude_animations[DudeAnimations::Kick1Recover.index()].clone(),
                false,
            ),
            Attack::Heavy2 => (
                assets.dude_animations[DudeAnimations::Kick2Recover.index()].clone(),
                false,
            ),
            Attack::Heavy3 => (
                assets.dude_animations[DudeAnimations::Kick3Recover.index()].clone(),
                false,
            ),
        },
        DudeState::Staggering => (
            assets.dude_animations[DudeAnimations::StaggerNormal.index()].clone(),
            false,
        ),
    }
}

pub fn dude_state_hitbox_frames(state: DudeState, attack: Attack) -> (usize, usize) {
    match state {
        DudeState::Idling => {
            error!("should never happen! idle doesn't have any hitbox frames");
            (0, 0)
        }
        DudeState::Running => {
            error!("should never happen! run doesn't have any hitbox frames");
            (0, 0)
        }
        DudeState::Attacking => match attack {
            Attack::Light1 => (0, 1),
            Attack::Light2 => (0, 1),
            Attack::Light3 => (1, 2),
            Attack::Heavy1 => (1, 2),
            Attack::Heavy2 => (1, 2),
            Attack::Heavy3 => (1, 2),
        },
        DudeState::Recovering => {
            error!("should never happen! recover doesn't have any hitbox frames");
            (0, 0)
        }
        DudeState::Staggering => {
            error!("should never happen! staggering doesn't have any hitbox frames");
            (0, 0)
        }
    }
}
