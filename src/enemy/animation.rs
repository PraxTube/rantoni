use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    state::{dude_state_animation, Attack, DudeState, Stagger},
    GameAssets,
};

use super::{state::EnemyStateSystemSet, Enemy};

fn flip_sprites(mut q_enemies: Query<(&mut Sprite, &mut Enemy, &Stagger)>) {
    for (mut sprite, enemy, stagger) in &mut q_enemies {
        if enemy.state_machine.state() == DudeState::Staggering {
            if stagger.direction.x == 0.0 {
                continue;
            }

            sprite.flip_x = stagger.direction.x > 0.0;
        }
    }
}

fn update_animations(
    assets: Res<GameAssets>,
    mut q_enemies: Query<(&mut AnimationPlayer2D, &Enemy, &Stagger)>,
) {
    for (mut animator, enemy, stagger) in &mut q_enemies {
        let (animation, repeat) = dude_state_animation(
            enemy.state_machine.state(),
            Attack::default(),
            stagger.state,
            &assets,
        );
        if repeat {
            animator.play(animation).repeat();
        } else {
            animator.play(animation);
        }
    }
}

pub struct EnemyAnimationPlugin;

impl Plugin for EnemyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flip_sprites, update_animations)
                .after(EnemyStateSystemSet)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
