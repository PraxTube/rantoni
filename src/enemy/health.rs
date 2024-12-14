use bevy::prelude::*;
use bevy_rancic::prelude::YSort;
use bevy_trickfilm::prelude::*;

use crate::{
    dude::Health,
    world::{DespawnLevelSystemSet, LevelChanged, WorldEntity, WorldSpatialData},
    GameAssets, GameState,
};

use super::Enemy;

const ANIMATOR_INSTANT_SPEED: f32 = 1000.0;

#[derive(Component)]
struct Bloodpile;

fn spawn_bloodpile(commands: &mut Commands, assets: &GameAssets, pos: &Vec2, finished: bool) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.blood_pile_animation.clone());

    if finished {
        animator.set_speed(ANIMATOR_INSTANT_SPEED);
    }

    commands.spawn((
        Bloodpile,
        WorldEntity,
        YSort(-500.0),
        animator,
        SpriteBundle {
            texture: assets.blood_pile.clone(),
            transform: Transform::from_translation(pos.extend(0.0)),
            ..default()
        },
        TextureAtlas::from(assets.blood_pile_layout.clone()),
    ));
}

fn spawn_bloodpiles_from_cached_data(
    mut commands: Commands,
    assets: Res<GameAssets>,
    world_data: Res<WorldSpatialData>,
) {
    for cached_pos in &world_data
        .cached_level_data()
        .unwrap_or_default()
        .bloodpiles
    {
        spawn_bloodpile(&mut commands, &assets, cached_pos, true);
    }
}

fn cache_bloodpiles(
    mut world_data: ResMut<WorldSpatialData>,
    q_bloodpiles: Query<&Transform, With<Bloodpile>>,
) {
    let mut cached_bloodpiles = Vec::new();
    for transform in &q_bloodpiles {
        cached_bloodpiles.push(transform.translation.truncate());
    }

    let mut cached_data = world_data.cached_previous_level_data().unwrap_or_default();
    cached_data.bloodpiles = cached_bloodpiles;
    world_data.update_cached_level_data(cached_data);
}

fn despawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_enemies: Query<(Entity, &Transform, &Health), With<Enemy>>,
) {
    for (entity, transform, health) in &q_enemies {
        if health.health == 0 {
            spawn_bloodpile(
                &mut commands,
                &assets,
                &transform.translation.truncate(),
                false,
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct EnemyHealthPlugin;

impl Plugin for EnemyHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TransitionLevel), cache_bloodpiles)
            .add_systems(
                Update,
                (
                    spawn_bloodpiles_from_cached_data
                        .run_if(
                            in_state(GameState::TransitionLevel)
                                .and_then(on_event::<LevelChanged>()),
                        )
                        .after(DespawnLevelSystemSet),
                    despawn_enemies,
                )
                    .run_if(resource_exists::<GameAssets>),
            )
            .add_systems(
                OnEnter(GameState::Restart),
                spawn_bloodpiles_from_cached_data.after(DespawnLevelSystemSet),
            );
    }
}
