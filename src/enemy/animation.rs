use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    assets::DudeAnimations,
    state::{DudeState, StaggerState},
    GameAssets,
};

use super::{state::EnemyStateSystemSet, Enemy};

fn flip_sprites(mut q_enemies: Query<(&mut Sprite, &mut Enemy)>) {
    for (mut sprite, enemy) in &mut q_enemies {
        if enemy.state == DudeState::Staggering {
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
            DudeState::Idling => assets.dude_animations[DudeAnimations::Idle.index()].clone(),
            DudeState::Staggering => match enemy.stagger.state {
                StaggerState::Normal => {
                    assets.dude_animations[DudeAnimations::StaggerNormal.index()].clone()
                }
                StaggerState::Flying => {
                    assets.dude_animations[DudeAnimations::StaggerFlying.index()].clone()
                }
            },
            _ => todo!(),
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
