use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::{dude_state_animation, ParryState, Stagger},
    GameAssets,
};

use super::{state::EnemyStateSystemSet, Enemy};

fn update_animations(
    assets: Res<GameAssets>,
    mut q_enemies: Query<(
        &mut Handle<Image>,
        &mut AnimationPlayer2D,
        &mut Enemy,
        &Stagger,
    )>,
) {
    for (mut enemy_texture, mut animator, mut enemy, stagger) in &mut q_enemies {
        let (texture, animation, repeat, animation_state) = dude_state_animation(
            &assets,
            enemy.state_machine.state(),
            enemy.state_machine.attack(),
            stagger.state,
            ParryState::default(),
            enemy.move_direction,
        );

        if &animation == animator.animation_clip() {
            return;
        }
        if !repeat && animation_state == enemy.state_machine.animation_state() {
            return;
        }
        enemy.state_machine.set_animation_state(animation_state);

        if repeat {
            animator.play(animation).repeat();
        } else {
            animator.play(animation);
        }

        *enemy_texture = texture;
    }
}

pub struct EnemyAnimationPlugin;

impl Plugin for EnemyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_animations,)
                .after(EnemyStateSystemSet)
                .before(AnimationPlayer2DSystemSet)
                .run_if(resource_exists::<GameAssets>),
        );
    }
}
