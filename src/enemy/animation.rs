use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{dude_state_animation_enemy, DudeState},
    GameAssets,
};

use super::Enemy;

fn update_animations(
    assets: Res<GameAssets>,
    mut q_enemies: Query<(
        &mut Handle<Image>,
        &Velocity,
        &mut AnimationPlayer2D,
        &mut Enemy,
    )>,
) {
    for (mut enemy_texture, velocity, mut animator, mut enemy) in &mut q_enemies {
        let stalk_direction = if enemy.state_machine.state() == DudeState::Stalking {
            velocity.linvel
        } else {
            Vec2::ZERO
        };
        let (texture, animation, repeat, animation_state) = dude_state_animation_enemy(
            &assets,
            enemy.state_machine.state(),
            enemy.state_machine.attack(),
            enemy.state_machine.stagger_state(),
            enemy.move_direction,
            stalk_direction,
        );

        if &animation == animator.animation_clip() {
            continue;
        }
        if !repeat && animation_state == enemy.state_machine.animation_state() {
            continue;
        }

        if repeat {
            if animation_state == enemy.state_machine.animation_state() {
                animator.play_continue(animation).repeat();
            } else {
                animator.play(animation).repeat();
            }
        } else {
            animator.play(animation);
        }

        enemy.state_machine.set_animation_state(animation_state);
        *enemy_texture = texture;
    }
}

pub struct EnemyAnimationPlugin;

impl Plugin for EnemyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_animations,)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
