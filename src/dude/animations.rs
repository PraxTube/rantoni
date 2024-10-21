use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{Attack, DudeState, ParryState, StaggerState};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum DudeAnimations {
    #[default]
    Idle,
    Run,
    Light1,
    Light1Recover,
    Light2,
    Light2Recover,
    Light3,
    Light3Recover,
    StaggerNormal,
    Heavy1,
    Heavy1Recover,
    Heavy2,
    Heavy2Recover,
    Heavy3,
    Heavy3Recover,
    StanceBreak,
    Parry,
    ParryFail,
    ParrySuccess,
    ParrySuccessRecover,
}

impl DudeAnimations {
    pub fn index(self) -> usize {
        self as usize
    }
}

/// Returns the index offset when taking 8-directional animations into account.
fn direction_index_offset(direction: Vec2) -> usize {
    let angle = direction.angle_between(Vec2::Y);

    if angle.abs() < PI / 8.0 {
        // Top
        0
    } else if angle.abs() < 3.0 / 8.0 * PI {
        // Diagonal Up
        if angle > 0.0 {
            // Top Right
            1
        } else {
            // Top Left
            7
        }
    } else if angle.abs() < 5.0 / 8.0 * PI {
        // Side
        if angle > 0.0 {
            // Right
            2
        } else {
            // Left
            6
        }
    } else if angle.abs() < 7.0 / 8.0 * PI {
        // Diagonal Down
        if angle > 0.0 {
            // Bottom Right
            3
        } else {
            // Bottom Left
            5
        }
    } else {
        // Bottom
        4
    }
}

fn match_attack_state(
    assets: &Res<GameAssets>,
    attack: Attack,
    direction: Vec2,
) -> (Handle<Image>, Handle<AnimationClip2D>, bool, DudeAnimations) {
    let animation_state = match attack {
        Attack::Light1 => DudeAnimations::Light1,
        Attack::Light2 => DudeAnimations::Light2,
        Attack::Light3 => DudeAnimations::Light3,
        Attack::Heavy1 => DudeAnimations::Heavy1,
        Attack::Heavy2 => DudeAnimations::Heavy2,
        Attack::Heavy3 => DudeAnimations::Heavy3,
    };
    let index = animation_state.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.dude_textures[index].clone(),
        assets.dude_animations[animation_index].clone(),
        false,
        animation_state,
    )
}

fn match_recover_state(
    assets: &Res<GameAssets>,
    attack: Attack,
    direction: Vec2,
) -> (Handle<Image>, Handle<AnimationClip2D>, bool, DudeAnimations) {
    let animation_state = match attack {
        Attack::Light1 => DudeAnimations::Light1Recover,
        Attack::Light2 => DudeAnimations::Light2Recover,
        Attack::Light3 => DudeAnimations::Light3Recover,
        Attack::Heavy1 => DudeAnimations::Heavy1Recover,
        Attack::Heavy2 => DudeAnimations::Heavy2Recover,
        Attack::Heavy3 => DudeAnimations::Heavy3Recover,
    };
    let index = animation_state.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.dude_textures[index].clone(),
        assets.dude_animations[animation_index].clone(),
        false,
        animation_state,
    )
}

fn match_stagger_state(
    assets: &Res<GameAssets>,
    stagger_state: StaggerState,
    direction: Vec2,
) -> (Handle<Image>, Handle<AnimationClip2D>, bool, DudeAnimations) {
    let animation_state = match stagger_state {
        StaggerState::Normal => DudeAnimations::StaggerNormal,
        StaggerState::StanceBreak => DudeAnimations::StanceBreak,
    };
    let index = animation_state.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.dude_textures[index].clone(),
        assets.dude_animations[animation_index].clone(),
        false,
        animation_state,
    )
}

fn match_parry_state(
    assets: &Res<GameAssets>,
    parry_state: ParryState,
    direction: Vec2,
) -> (Handle<Image>, Handle<AnimationClip2D>, bool, DudeAnimations) {
    let animation_state = match parry_state {
        ParryState::Start => DudeAnimations::Parry,
        ParryState::Fail => DudeAnimations::ParryFail,
        ParryState::Success => DudeAnimations::ParrySuccess,
        ParryState::Recover => DudeAnimations::ParrySuccessRecover,
    };
    let index = animation_state.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.dude_textures[index].clone(),
        assets.dude_animations[animation_index].clone(),
        false,
        animation_state,
    )
}

pub fn dude_state_animation(
    assets: &Res<GameAssets>,
    state: DudeState,
    attack: Attack,
    stagger_state: StaggerState,
    parry_state: ParryState,
    direction: Vec2,
) -> (Handle<Image>, Handle<AnimationClip2D>, bool, DudeAnimations) {
    match state {
        DudeState::Idling => {
            let index = DudeAnimations::Idle.index();
            let animation_index = index * 8 + direction_index_offset(direction);
            (
                assets.dude_textures[index].clone(),
                assets.dude_animations[animation_index].clone(),
                true,
                DudeAnimations::Idle,
            )
        }
        DudeState::Running => {
            let index = DudeAnimations::Run.index();
            let animation_index = index * 8 + direction_index_offset(direction);
            (
                assets.dude_textures[index].clone(),
                assets.dude_animations[animation_index].clone(),
                true,
                DudeAnimations::Run,
            )
        }
        DudeState::Attacking => match_attack_state(assets, attack, direction),
        DudeState::Recovering => match_recover_state(assets, attack, direction),
        DudeState::Staggering => match_stagger_state(assets, stagger_state, direction),
        DudeState::Parrying => match_parry_state(assets, parry_state, direction),
    }
}

/// Corresponding frame of when to spawn the attack arc of each attack animation.
pub fn dude_state_hitbox_start_frame(state: DudeState, attack: Attack) -> usize {
    match state {
        DudeState::Attacking => match attack {
            Attack::Light1 => 3,
            Attack::Light2 => 4,
            Attack::Light3 => 2,
            Attack::Heavy1 => 5,
            Attack::Heavy2 => 11,
            Attack::Heavy3 => 11,
        },
        _ => {
            error!(
                "should never happen, called hitbox start frame in state '{:?}'",
                state
            );
            0
        }
    }
}
