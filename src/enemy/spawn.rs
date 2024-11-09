use bevy::{color::palettes::css::RED, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rancic::prelude::{YSort, YSortChild};
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use generate_world_collisions::TILE_SIZE;

use crate::{
    dude::DudeAnimations,
    world::{
        collisions::{spawn_hurtbox_collision, Hurtbox, HurtboxType, ENEMY_GROUP, WORLD_GROUP},
        LevelChanged, PathfindingSource, WorldEntity, WorldSpatialData,
    },
    GameAssets, GameState,
};

use super::Enemy;

fn spawn_dummy_enemy(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec2) {
    let entity = commands
        .spawn((
            Enemy::default(),
            WorldEntity,
            // TODO: Bundle into some kind of convenience bundle so you don't forget them
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Damping {
                linear_damping: 100.0,
                ..default()
            },
            YSort(0.0),
            SpriteBundle {
                transform: Transform::from_translation(pos.extend(0.0)),
                texture: assets.dude_textures[0].clone(),
                sprite: Sprite {
                    color: RED.into(),
                    ..default()
                },
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let hurtbox_normal = spawn_hurtbox_collision(
        commands,
        Hurtbox::new(entity, HurtboxType::Normal, ENEMY_GROUP),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(8.0, 24.0),
    );
    let hurtbox_fallen = spawn_hurtbox_collision(
        commands,
        Hurtbox::new(entity, HurtboxType::Fallen, ENEMY_GROUP),
        Vec2::new(0.0, -16.0),
        Collider::cuboid(20.0, 14.0),
    );

    let collider = commands
        .spawn((
            PathfindingSource::new(entity),
            Collider::ball(10.0),
            CollisionGroups::new(WORLD_GROUP | ENEMY_GROUP, WORLD_GROUP),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    let shadow = commands
        .spawn((
            YSortChild(-100.0),
            SpriteBundle {
                texture: assets.dude_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -18.0, 0.0)),
                ..default()
            },
        ))
        .id();

    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[DudeAnimations::Idle.index()].clone())
        .repeat();

    commands.entity(entity).insert(animator).push_children(&[
        collider,
        hurtbox_normal,
        hurtbox_fallen,
        shadow,
    ]);
}

fn spawn_dummy_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    world_data: Res<WorldSpatialData>,
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
        // TODO: Factor this out, probably some kind of config file that bridges identifiers from
        // LDTK with Bevy
        if layer_instance.identifier != "Enemies" {
            continue;
        }

        for entity_instance in layer_instance.entity_instances {
            let pos = Vec2::new(
                entity_instance.px.x as f32,
                world_data.level_dimensions().y as f32 * TILE_SIZE - entity_instance.px.y as f32,
            );
            info!("{}", pos);
            spawn_dummy_enemy(&mut commands, &assets, pos);
        }
    }
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_dummy_enemies.run_if(on_event::<LevelChanged>()),)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
