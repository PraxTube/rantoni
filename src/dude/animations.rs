use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{Attack, DudeState, ParryState, StaggerState};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum PlayerAnimations {
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
    StaggerNormalRecover,
    Heavy1,
    Heavy1Recover,
    Heavy2,
    Heavy2Recover,
    Heavy3,
    Heavy3Recover,
    StanceBreak,
    StanceBreakRecover,
    Parry,
    ParryFail,
    ParrySuccess,
    Dash,
    Fall,
    FallRecover,
    Dropkick,
    DropkickRecover,
    Hammerfist,
    HammerfistRecover,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum EnemyAnimations {
    #[default]
    Idle,
    Run,
    Light1,
    Light1Recover,
    Heavy1,
    Heavy1Recover,
    StaggerNormal,
    StaggerNormalRecover,
    StanceBreak,
    StanceBreakRecover,
    Fall,
    FallRecover,
    Stalking,
    StalkingForward,
    StalkingRight,
    StalkingBack,
    StalkingLeft,
}

impl PlayerAnimations {
    pub fn index(self) -> usize {
        self as usize
    }
}

impl EnemyAnimations {
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

fn get_animation_data_player(
    assets: &Res<GameAssets>,
    player_animation: PlayerAnimations,
    direction: Vec2,
    repeat: bool,
) -> (
    Handle<Image>,
    Handle<AnimationClip2D>,
    bool,
    PlayerAnimations,
) {
    let index = player_animation.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.dude_textures[index].clone(),
        assets.dude_animations[animation_index].clone(),
        repeat,
        player_animation,
    )
}

fn get_animation_data_enemy(
    assets: &Res<GameAssets>,
    enemy_animation: EnemyAnimations,
    direction: Vec2,
    repeat: bool,
) -> (
    Handle<Image>,
    Handle<AnimationClip2D>,
    bool,
    EnemyAnimations,
) {
    let index = enemy_animation.index();
    let animation_index = index * 8 + direction_index_offset(direction);

    (
        assets.enemy_goon_textures[index].clone(),
        assets.enemy_goon_animations[animation_index].clone(),
        repeat,
        enemy_animation,
    )
}

pub fn dude_state_animation_player(
    assets: &Res<GameAssets>,
    state: DudeState,
    attack: Attack,
    stagger_state: StaggerState,
    direction: Vec2,
) -> (
    Handle<Image>,
    Handle<AnimationClip2D>,
    bool,
    PlayerAnimations,
) {
    match state {
        DudeState::Idling => {
            get_animation_data_player(assets, PlayerAnimations::Idle, direction, true)
        }
        DudeState::Running => {
            get_animation_data_player(assets, PlayerAnimations::Run, direction, true)
        }
        DudeState::Attacking => {
            let dude_animation = match attack {
                Attack::Light1 => PlayerAnimations::Light1,
                Attack::Light2 => PlayerAnimations::Light2,
                Attack::Light3 => PlayerAnimations::Light3,
                Attack::Heavy1 => PlayerAnimations::Heavy1,
                Attack::Heavy2 => PlayerAnimations::Heavy2,
                Attack::Heavy3 => PlayerAnimations::Heavy3,
                Attack::Dropkick => PlayerAnimations::Dropkick,
                Attack::Hammerfist => PlayerAnimations::Hammerfist,
            };
            get_animation_data_player(assets, dude_animation, direction, false)
        }
        DudeState::Recovering => {
            let dude_animation = match attack {
                Attack::Light1 => PlayerAnimations::Light1Recover,
                Attack::Light2 => PlayerAnimations::Light2Recover,
                Attack::Light3 => PlayerAnimations::Light3Recover,
                Attack::Heavy1 => PlayerAnimations::Heavy1Recover,
                Attack::Heavy2 => PlayerAnimations::Heavy2Recover,
                Attack::Heavy3 => PlayerAnimations::Heavy3Recover,
                Attack::Dropkick => PlayerAnimations::DropkickRecover,
                Attack::Hammerfist => PlayerAnimations::HammerfistRecover,
            };
            get_animation_data_player(assets, dude_animation, direction, false)
        }
        DudeState::Staggering => {
            let dude_animation = match stagger_state {
                StaggerState::Normal => PlayerAnimations::StaggerNormal,
                StaggerState::StanceBreak => PlayerAnimations::StanceBreak,
                StaggerState::Fall => PlayerAnimations::Fall,
                StaggerState::NormalRecover => PlayerAnimations::StaggerNormalRecover,
                StaggerState::StanceBreakRecover => PlayerAnimations::StanceBreakRecover,
                StaggerState::FallRecover => PlayerAnimations::FallRecover,
            };

            let direction = if dude_animation == PlayerAnimations::Fall
                || dude_animation == PlayerAnimations::FallRecover
            {
                // We only care about one of the 8 directions, but because of how the whole pipeline is
                // set up we just keep the other 7 useless animations, it's a minor issue but
                // absolutely not worth the optimization.
                Vec2::Y
            } else {
                direction
            };

            get_animation_data_player(assets, dude_animation, direction, false)
        }
        DudeState::Parrying(parry_state) => {
            let dude_animation = match parry_state {
                ParryState::Start => PlayerAnimations::Parry,
                ParryState::Fail => PlayerAnimations::ParryFail,
                ParryState::Success => PlayerAnimations::ParrySuccess,
            };
            get_animation_data_player(assets, dude_animation, direction, false)
        }
        DudeState::Stalking => {
            get_animation_data_player(assets, PlayerAnimations::Idle, direction, true)
        }
        DudeState::Dashing => {
            error!("this should never happen! You are not allowed to call this function when in dashing state!");
            get_animation_data_player(assets, PlayerAnimations::Idle, direction, false)
        }
    }
}

pub fn dude_state_animation_enemy(
    assets: &Res<GameAssets>,
    state: DudeState,
    attack: Attack,
    stagger_state: StaggerState,
    direction: Vec2,
    stalk_direction: Vec2,
) -> (
    Handle<Image>,
    Handle<AnimationClip2D>,
    bool,
    EnemyAnimations,
) {
    match state {
        DudeState::Idling => {
            get_animation_data_enemy(assets, EnemyAnimations::Idle, direction, true)
        }
        DudeState::Running => {
            get_animation_data_enemy(assets, EnemyAnimations::Run, direction, true)
        }
        DudeState::Attacking => {
            let animation = match attack {
                Attack::Light1 => EnemyAnimations::Light1,
                Attack::Heavy1 => EnemyAnimations::Heavy1,
                _ => {
                    error!("enemies are only allowed to have one light and one heavy attack, should never happen");
                    EnemyAnimations::Light1
                }
            };
            get_animation_data_enemy(assets, animation, direction, false)
        }
        DudeState::Recovering => {
            let animation = match attack {
                Attack::Light1 => EnemyAnimations::Light1Recover,
                Attack::Heavy1 => EnemyAnimations::Heavy1Recover,
                _ => {
                    error!("enemies are only allowed to have one light and one heavy attack, should never happen");
                    EnemyAnimations::Light1Recover
                }
            };
            get_animation_data_enemy(assets, animation, direction, false)
        }
        DudeState::Staggering => {
            let animation = match stagger_state {
                StaggerState::Normal => EnemyAnimations::StaggerNormal,
                StaggerState::StanceBreak => EnemyAnimations::StanceBreak,
                StaggerState::Fall => EnemyAnimations::Fall,
                StaggerState::NormalRecover => EnemyAnimations::StaggerNormalRecover,
                StaggerState::StanceBreakRecover => EnemyAnimations::StanceBreakRecover,
                StaggerState::FallRecover => EnemyAnimations::FallRecover,
            };

            let direction = if animation == EnemyAnimations::Fall
                || animation == EnemyAnimations::FallRecover
            {
                // We only care about one of the 8 directions, but because of how the whole pipeline is
                // set up we just keep the other 7 useless animations, it's a minor issue but
                // absolutely not worth the optimization.
                Vec2::Y
            } else {
                direction
            };

            get_animation_data_enemy(assets, animation, direction, false)
        }
        DudeState::Stalking => {
            let angle = direction.angle_between(stalk_direction);

            let animation = if angle.abs() < PI / 4.0 {
                EnemyAnimations::StalkingForward
            } else if angle.abs() < 3.0 / 4.0 * PI {
                if angle > 0.0 {
                    EnemyAnimations::StalkingRight
                } else {
                    EnemyAnimations::StalkingLeft
                }
            } else {
                EnemyAnimations::StalkingBack
            };

            get_animation_data_enemy(assets, animation, direction, true)
        }
        DudeState::Parrying(_) => {
            error!("called animation data on enemy with parry state, should never happen");
            get_animation_data_enemy(assets, EnemyAnimations::Idle, direction, false)
        }
        DudeState::Dashing => {
            error!("this should never happen! You are not allowed to call this function when in dashing state!");
            get_animation_data_enemy(assets, EnemyAnimations::Idle, direction, false)
        }
    }
}

pub fn dude_dashing_sprites(assets: &Res<GameAssets>, direction: Vec2) -> (Handle<Image>, usize) {
    let (texture, _, _, _) =
        get_animation_data_player(assets, PlayerAnimations::Dash, direction, false);
    (texture, direction_index_offset(direction))
}
