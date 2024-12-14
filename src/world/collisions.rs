use bevy::{color::palettes::css::LIME, prelude::*};
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use bevy_trickfilm::prelude::*;

use crate::{dude::Attack, GameAssets, GameState};

use super::{
    map::WorldSpatialData, quat_from_vec2, DespawnLevelSystemSet, LevelChanged, YSortChild,
    COLLIDER_COLOR_BLACK, COLLIDER_COLOR_WHITE, COLLISION_GROUPS_NONE,
};

pub const HURTBOX_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const WORLD_GROUP: Group = Group::GROUP_3;
pub const PLAYER_GROUP: Group = Group::GROUP_4;
pub const ENEMY_GROUP: Group = Group::GROUP_5;

const HITBOX_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(HITBOX_GROUP, HURTBOX_GROUP);
pub const HURTBOX_COLLISION_GROUPS: CollisionGroups =
    CollisionGroups::new(HURTBOX_GROUP, HITBOX_GROUP);

#[derive(PartialEq, Eq, Clone)]
pub enum HitboxType {
    Player(Attack),
    Enemy(Attack),
}

#[derive(Component, Clone)]
pub struct Hitbox {
    pub root_entity: Entity,
    pub hitbox_type: HitboxType,
    pub offset: Vec2,
    pub attack_direction: Vec2,
}

#[derive(Component, Clone)]
pub struct Hurtbox {
    pub root_entity: Entity,
}

#[derive(Component)]
struct WorldCollision;

#[derive(Component)]
pub struct AttackArcGFX;

#[derive(Component)]
pub struct AttackArc {
    timer: Timer,
    dir: Vec2,
}

#[derive(Event)]
pub struct HitboxHurtboxEvent {
    pub hitbox: Hitbox,
    pub hurtbox: Hurtbox,
}

impl Hitbox {
    pub fn new(
        root_entity: Entity,
        hitbox_type: HitboxType,
        offset: Vec2,
        attack_direction: Vec2,
    ) -> Self {
        Self {
            root_entity,
            hitbox_type,
            offset,
            attack_direction,
        }
    }
}

impl Hurtbox {
    pub fn new(root_entity: Entity) -> Self {
        Self { root_entity }
    }
}

impl AttackArc {
    fn new(dir: Vec2) -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            dir,
        }
    }

    pub fn dir(&self) -> Vec2 {
        self.dir
    }
}

pub fn spawn_hitbox_collision(
    commands: &mut Commands,
    hitbox: Hitbox,
    collider: Collider,
) -> Entity {
    let transform = Transform::from_translation(hitbox.offset.extend(0.0));
    commands
        .spawn((
            hitbox.clone(),
            collider,
            Sensor,
            HITBOX_COLLISION_GROUPS,
            COLLIDER_COLOR_WHITE,
            TransformBundle::from_transform(transform),
        ))
        .id()
}

pub fn spawn_hurtbox_collision(
    commands: &mut Commands,
    hurtbox: Hurtbox,
    offset: Vec2,
    collider: Collider,
) -> Entity {
    commands
        .spawn((
            hurtbox,
            collider,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            HURTBOX_COLLISION_GROUPS,
            COLLIDER_COLOR_WHITE,
            TransformBundle::from_transform(Transform::from_translation(offset.extend(0.0))),
        ))
        .id()
}

pub fn spawn_attack_effect(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    entity: Entity,
    direction: Vec2,
    hitbox_type: HitboxType,
) {
    let (hitbox_offset, collider, direction_magnitude, pos_offset) = match hitbox_type {
        HitboxType::Player(attack) => attack.effect_position_data(),
        HitboxType::Enemy(attack) => attack.effect_position_data(),
    };
    let (texture, layout, animation, with_rotation) = match hitbox_type {
        HitboxType::Player(attack) => attack.effect_animation_data(assets),
        HitboxType::Enemy(attack) => attack.effect_animation_data(assets),
    };
    let hitbox = spawn_hitbox_collision(
        commands,
        Hitbox::new(entity, hitbox_type, hitbox_offset, direction),
        collider,
    );

    let mut animator = AnimationPlayer2D::default();
    animator.play(animation);

    let dir = direction.normalize_or_zero();
    let pos = pos_offset + dir * direction_magnitude;
    let transform = if with_rotation {
        Transform::from_translation(pos.extend(0.0)).with_rotation(quat_from_vec2(direction))
    } else {
        Transform::from_translation(pos.extend(0.0))
    };

    let gfx = commands
        .spawn((
            AttackArcGFX,
            animator,
            SpriteBundle {
                texture,
                ..default()
            },
            TextureAtlas::from(layout),
        ))
        .id();

    let attack_arc = commands
        .spawn((
            AttackArc::new(dir),
            YSortChild(10.0),
            SpatialBundle::from_transform(transform.with_scale(Vec3::ONE * 1.5)),
        ))
        .push_children(&[hitbox, gfx])
        .id();
    commands.entity(entity).add_child(attack_arc);
}

fn disable_attack_arc_hitboxes(
    time: Res<Time>,
    mut q_attack_arcs: Query<(&Children, &mut AttackArc)>,
    mut q_hitboxes: Query<(&mut CollisionGroups, &mut ColliderDebugColor), With<Hitbox>>,
) {
    for (children, mut attack_arc) in &mut q_attack_arcs {
        attack_arc.timer.tick(time.delta());
        if !attack_arc.timer.just_finished() {
            continue;
        }

        for child in children {
            if let Ok((mut collision_groups, mut collider_color)) = q_hitboxes.get_mut(*child) {
                *collision_groups = COLLISION_GROUPS_NONE;
                *collider_color = COLLIDER_COLOR_BLACK;
            }
        }
    }
}

fn despawn_attack_arcs(
    mut commands: Commands,
    q_attack_arc_gfxs: Query<(&Parent, &AnimationPlayer2D), With<AttackArcGFX>>,
    q_attack_arcs: Query<Entity, With<AttackArc>>,
) {
    for (parent, animator) in &q_attack_arc_gfxs {
        if animator.just_finished() {
            let Ok(entity) = q_attack_arcs.get(parent.get()) else {
                continue;
            };
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_map_collisions(mut commands: Commands, world_data: Res<WorldSpatialData>) {
    for poly in world_data.collider_polygons() {
        commands.spawn((
            WorldCollision,
            CollisionGroups::new(WORLD_GROUP, WORLD_GROUP | ENEMY_GROUP | PLAYER_GROUP),
            Collider::convex_hull(poly).expect(
                "polygon should be convertable to convex hull, something went really wrong",
            ),
            ColliderDebugColor(LIME.into()),
        ));
    }
}

fn despawn_map_collisions(
    mut commands: Commands,
    q_world_collisions: Query<Entity, With<WorldCollision>>,
) {
    for entity in &q_world_collisions {
        commands.entity(entity).despawn_recursive();
    }
}

fn relay_hitbox_hurtbox_events(
    q_hitboxes: Query<&Hitbox>,
    q_hurtboxes: Query<&Hurtbox>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    mut ev_hitbox_hurtbox: EventWriter<HitboxHurtboxEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target, flags) = match ev {
            CollisionEvent::Started(source, target, flags) => (source, target, flags),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        // None of the colliders are sensors, so it can't be hitbox & hurtbox collision.
        if *flags & CollisionEventFlags::SENSOR != CollisionEventFlags::SENSOR {
            continue;
        }

        let Ok(hitbox) = q_hitboxes.get(*source).or(q_hitboxes.get(*target)) else {
            continue;
        };

        let Ok(hurtbox) = q_hurtboxes.get(*source).or(q_hurtboxes.get(*target)) else {
            continue;
        };

        ev_hitbox_hurtbox.send(HitboxHurtboxEvent {
            hitbox: hitbox.clone(),
            hurtbox: hurtbox.clone(),
        });
    }
}

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitboxHurtboxEvent>()
            .add_systems(PreUpdate, relay_hitbox_hurtbox_events)
            .add_systems(
                Update,
                (disable_attack_arc_hitboxes, despawn_attack_arcs)
                    .run_if(not(in_state(GameState::AssetLoading))),
            )
            .add_systems(
                Update,
                (
                    despawn_map_collisions.in_set(DespawnLevelSystemSet),
                    spawn_map_collisions.after(DespawnLevelSystemSet),
                )
                    .run_if(
                        in_state(GameState::TransitionLevel).and_then(on_event::<LevelChanged>()),
                    ),
            )
            .add_systems(
                OnEnter(GameState::Restart),
                (
                    despawn_map_collisions.in_set(DespawnLevelSystemSet),
                    spawn_map_collisions.after(DespawnLevelSystemSet),
                ),
            );
    }
}
