use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    assets::events::SpawnHitboxEvent,
    dude::{dude_state_animation_enemy, DudeState},
    GameAssets,
};

use super::{state::EnemyStateSystemSet, Enemy};

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
        let direction = match enemy.state_machine.state() {
            DudeState::Idling
            | DudeState::Running
            | DudeState::Staggering
            | DudeState::Stalking => enemy.move_direction,
            DudeState::Attacking | DudeState::Recovering => enemy.state_machine.attack_direction(),
            DudeState::Parrying(_) | DudeState::Dashing => {
                panic!("enemy must never go into these states. Should never happen")
            }
            DudeState::Dying => continue,
        };

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
            direction,
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

fn disable_can_move_during_attack(
    mut q_enemies: Query<&mut Enemy>,
    mut ev_spawn_hitbox: EventReader<SpawnHitboxEvent>,
) {
    for ev in ev_spawn_hitbox.read() {
        let Ok(mut enemy) = q_enemies.get_mut(*ev.target) else {
            continue;
        };

        enemy.state_machine.disable_can_move_during_attack();
    }
}

pub struct EnemyAnimationPlugin;

impl Plugin for EnemyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            disable_can_move_during_attack.after(EnemyStateSystemSet),
        )
        .add_systems(
            PostUpdate,
            (update_animations,)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
