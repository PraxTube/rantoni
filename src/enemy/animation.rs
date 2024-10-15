use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    state::{dude_state_animation, Attack, Stagger},
    GameAssets,
};

use super::{state::EnemyStateSystemSet, Enemy};

fn update_animations(
    assets: Res<GameAssets>,
    mut q_enemies: Query<(&mut Handle<Image>, &mut AnimationPlayer2D, &Enemy, &Stagger)>,
) {
    for (mut enemy_texture, mut animator, enemy, stagger) in &mut q_enemies {
        let (texture, animation, repeat, _) = dude_state_animation(
            &assets,
            enemy.state_machine.state(),
            Attack::default(),
            stagger.state,
            enemy.move_direction,
        );

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
