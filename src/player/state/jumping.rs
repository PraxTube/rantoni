use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{assets::PLAYER_SPRITE_SIZE, dude::Attack, player::Player};

use super::PlayerStateSystemSet;

const JUMP_HEIGHT: f32 = 30.0 / PLAYER_SPRITE_SIZE as f32;

#[derive(Default)]
pub struct Jumping {
    duration: f32,
    elapsed: f32,
}

impl Jumping {
    fn x(&self) -> f32 {
        if self.duration == 0.0 {
            return 0.0;
        }

        self.elapsed / self.duration
    }

    pub fn sprite_y_offset(&self) -> f32 {
        if self.elapsed >= self.duration || self.duration == 0.0 {
            return 0.0;
        }
        let x = self.x() * 0.7 + 0.3;
        let y = (x * PI).sin();
        JUMP_HEIGHT * y
    }

    pub fn speed(&self) -> f32 {
        (1.0 - self.x().powi(5)).max(0.7)
    }

    pub fn tick_timer(&mut self, delta: Duration) {
        self.elapsed += delta.as_secs_f32();
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn reset_timer(&mut self) {
        self.elapsed = 0.0;
    }
}

fn tick_timers(time: Res<Time>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.attack_eq(Attack::Dropkick)
            || player.state_machine.attack_eq(Attack::Hammerfist)
        {
            player.state_machine.tick_jumping_timer(time.delta());
        } else {
            player.state_machine.reset_jumping_timer();
        }
    }
}

fn set_durations(mut q_players: Query<(&AnimationPlayer2D, &mut Player)>) {
    for (animator, mut player) in &mut q_players {
        if player.state_machine.jumping_duration() != 0.0 {
            continue;
        }
        if !player.state_machine.attack_eq(Attack::Hammerfist)
            && !player.state_machine.attack_eq(Attack::Dropkick)
        {
            continue;
        }

        if let Some(duration) = animator.duration() {
            player.state_machine.set_jumping_duration(duration);
        }
    }
}

pub struct PlayerJumpingStatePlugin;

impl Plugin for PlayerJumpingStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (set_durations, tick_timers).before(PlayerStateSystemSet),
        );
    }
}
