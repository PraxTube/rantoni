use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{state::StaggerState, GameAssets};

use super::{
    state::{EnemyState, EnemyStateSystemSet},
    Enemy,
};

fn flip_sprites(mut q_enemies: Query<(&mut Sprite, &mut Enemy)>) {
    for (mut sprite, enemy) in &mut q_enemies {
        if enemy.state == EnemyState::Staggering {
            if enemy.stagger.direction.x == 0.0 {
                continue;
            }

            sprite.flip_x = enemy.stagger.direction.x > 0.0;
        }
    }
}

fn update_animations(
    assets: Res<GameAssets>,
    mut q_enemies: Query<(&mut AnimationPlayer2D, &Enemy)>,
) {
    for (mut animator, enemy) in &mut q_enemies {
        let animation = match enemy.state {
            EnemyState::Idling => assets.player_animations[0].clone(),
            EnemyState::Staggering => match enemy.stagger.state {
                StaggerState::Normal => assets.player_animations[6].clone(),
                StaggerState::Flying => assets.player_animations[13].clone(),
            },
        };

        animator.play(animation).repeat();
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
