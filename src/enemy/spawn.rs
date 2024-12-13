use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use generate_world_collisions::ENEMY_LAYER_IDENTIFIER;

use crate::{
    dude::{EnemyAnimations, Health},
    world::{
        collisions::{spawn_hurtbox_collision, Hurtbox, ENEMY_GROUP, PLAYER_GROUP, WORLD_GROUP},
        CachedEnemy, CachedLevelData, DespawnLevelSystemSet, LevelChanged, PathfindingSource,
        WorldEntity, WorldSpatialData, YSort, YSortChild,
    },
    GameAssets, GameState,
};

use super::Enemy;

pub const COLLIDER_RADIUS: f32 = 16.0;

fn spawn_dummy_enemy(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let entity = commands
        .spawn((
            Enemy::default(),
            Health::new(34),
            WorldEntity,
            // TODO: Bundle into some kind of convenience bundle so you don't forget them
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            YSort(0.0),
            SpriteBundle {
                transform: Transform::from_translation(pos.extend(0.0)),
                texture: assets.enemy_goon_textures[0].clone(),
                ..default()
            },
            TextureAtlas::from(assets.enemy_goon_layout.clone()),
        ))
        .id();

    let hurtbox = spawn_hurtbox_collision(
        commands,
        Hurtbox::new(entity),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(20.0, 40.0),
    );

    let collider = commands
        .spawn((
            PathfindingSource::new(entity),
            Collider::ball(COLLIDER_RADIUS),
            CollisionGroups::new(ENEMY_GROUP, WORLD_GROUP | PLAYER_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    let shadow = commands
        .spawn((
            YSortChild(-100.0),
            SpriteBundle {
                texture: assets.enemy_goon_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -18.0, 0.0)),
                ..default()
            },
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.enemy_goon_animations[EnemyAnimations::Idle.index()].clone())
        .repeat();

    commands
        .entity(entity)
        .insert(animator)
        .push_children(&[collider, hurtbox, shadow]);
}

fn spawn_enemies_from_ldtk(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    ldtk_project_assets: &Res<Assets<LdtkProject>>,
    world_data: &Res<WorldSpatialData>,
) {
    let project = ldtk_project_assets
        .get(&assets.map)
        .expect("ldtk project should be loaded at this point, maybe time was not enough, is the project really big?");

    let layer_instances = project.worlds()[world_data.world_index()]
        .levels[world_data.level_index()]
        .layer_instances
        .clone()
        .expect("layer instances should never be null, it's okay to be empty, but not null, probably issue with 'separate levels' option");

    for layer_instance in layer_instances {
        if layer_instance.identifier != ENEMY_LAYER_IDENTIFIER {
            continue;
        }

        for entity_instance in layer_instance.entity_instances {
            let pos = world_data.pixel_coords_to_translation(entity_instance.px);
            spawn_dummy_enemy(commands, assets, pos);
        }
    }
}

fn spawn_enemies_from_cached_data(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    cached_data: &CachedLevelData,
) {
    for cached_enemy in &cached_data.enemies {
        spawn_dummy_enemy(commands, assets, cached_enemy.pos);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    world_data: Res<WorldSpatialData>,
) {
    match world_data.cached_level_data() {
        Some(cached_data) => spawn_enemies_from_cached_data(&mut commands, &assets, &cached_data),
        None => spawn_enemies_from_ldtk(&mut commands, &assets, &ldtk_project_assets, &world_data),
    };
}

fn cache_enemies(
    mut world_data: ResMut<WorldSpatialData>,
    q_enemies: Query<&Transform, With<Enemy>>,
) {
    let mut cached_enemies = Vec::new();
    for transform in &q_enemies {
        cached_enemies.push(CachedEnemy {
            pos: transform.translation.truncate(),
        });
    }

    let mut cached_data = world_data.cached_previous_level_data().unwrap_or_default();
    cached_data.enemies = cached_enemies;
    world_data.update_cached_level_data(cached_data);
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(in_state(GameState::TransitionLevel).and_then(on_event::<LevelChanged>()))
                .after(DespawnLevelSystemSet),
        )
        .add_systems(
            OnEnter(GameState::Restart),
            spawn_enemies.after(DespawnLevelSystemSet),
        )
        .add_systems(OnEnter(GameState::TransitionLevel), cache_enemies);
    }
}
