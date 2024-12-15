use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use generate_world_collisions::PLAYER_LAYER_IDENTIFIER;

use crate::{
    dude::{Health, PlayerAnimations},
    world::{
        collisions::{spawn_hurtbox_collision, Hurtbox},
        CachedPlayer, DespawnLevelSystemSet, PathfindingTarget, WorldSpatialData, YSort,
        YSortChild,
    },
    GameAssets, GameState,
};

use super::{collisions::DEFAULT_PLAYER_COLLISION_GROUPS, Player, HEALTH};

fn spawn_player_from_data(commands: &mut Commands, assets: &GameAssets, pos: Vec2, health: u32) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.dude_animations[PlayerAnimations::Idle.index()].clone())
        .repeat();

    let entity = commands
        .spawn((
            Player::default(),
            Health::new(health),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            Dominance::group(1),
            animator,
            YSort(0.0),
            SpriteBundle {
                texture: assets.dude_textures[0].clone(),
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            TextureAtlas::from(assets.dude_layout.clone()),
        ))
        .id();

    let collider = commands
        .spawn((
            PathfindingTarget {
                root_entity: entity,
            },
            Collider::ball(16.0),
            DEFAULT_PLAYER_COLLISION_GROUPS,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -16.0, 0.0,
            ))),
        ))
        .id();

    let hurtbox = spawn_hurtbox_collision(
        commands,
        Hurtbox::new(entity),
        Vec2::new(0.0, 0.0),
        Collider::cuboid(10.0, 30.0),
    );

    let shadow = commands
        .spawn((
            YSortChild(-100.0),
            SpriteBundle {
                texture: assets.dude_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -27.0, 0.0)),
                ..default()
            },
        ))
        .id();

    commands
        .entity(entity)
        .push_children(&[collider, hurtbox, shadow]);
}

fn spawn_player_from_ldtk(
    commands: &mut Commands,
    assets: &GameAssets,
    ldtk_project_assets: &Assets<LdtkProject>,
    world_data: &WorldSpatialData,
) -> CachedPlayer {
    let project = ldtk_project_assets
        .get(&assets.map)
        .expect("ldtk project should be loaded at this point, maybe time was not enough, is the project really big?");

    let layer_instances = project.worlds()[world_data.world_index()]
        .levels[world_data.level_index()]
        .layer_instances
        .clone()
        .expect("layer instances should never be null, it's okay to be empty, but not null, probably issue with 'separate levels' option");

    for layer_instance in layer_instances {
        if layer_instance.identifier != PLAYER_LAYER_IDENTIFIER {
            continue;
        }

        assert_eq!(layer_instance.entity_instances.len(), 1);

        let pos = world_data.pixel_coords_to_translation(layer_instance.entity_instances[0].px);
        spawn_player_from_data(commands, assets, pos, HEALTH);
        return CachedPlayer {
            pos,
            health: HEALTH,
        };
    }
    panic!("Failed to get player from ldtk layer instances. This most likely means you either forgot to place a player position or you are not started the game from level 0.");
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut world_data: ResMut<WorldSpatialData>,
) {
    match world_data.cached_player() {
        Some(cached_player) => spawn_player_from_data(
            &mut commands,
            &assets,
            cached_player.pos,
            cached_player.health,
        ),
        None => {
            let new_cached_player =
                spawn_player_from_ldtk(&mut commands, &assets, &ldtk_project_assets, &world_data);
            world_data.set_cached_player(new_cached_player);
        }
    }
}

fn despawn_players(mut commands: Commands, q_players: Query<Entity, With<Player>>) {
    for entity in &q_players {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Restart),
            (
                despawn_players.in_set(DespawnLevelSystemSet),
                spawn_player.after(DespawnLevelSystemSet),
            ),
        );
    }
}
