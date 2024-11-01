use bevy::{color::palettes::css::LIME, prelude::*};
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{dude::Attack, GameAssets, GameState};

use super::map::MapPolygonData;

pub const HURTBOX_GROUP: Group = Group::GROUP_1;
pub const HITBOX_GROUP: Group = Group::GROUP_2;
pub const WORLD_GROUP: Group = Group::GROUP_3;
pub const PLAYER_GROUP: Group = Group::GROUP_4;
pub const ENEMY_GROUP: Group = Group::GROUP_5;

#[derive(PartialEq, Eq, Clone)]
pub enum HitboxType {
    Player(Attack),
    Enemy(Attack),
}

#[derive(Component, Clone)]
pub struct Hitbox {
    pub root_entity: Entity,
    pub hitbox_type: HitboxType,
    pub memberships: Group,
    pub offset: Vec2,
    filters: Group,
}

#[derive(Component, Clone)]
pub struct Hurtbox {
    pub root_entity: Entity,
    pub hurtbox_type: HurtboxType,
    pub collision_groups: CollisionGroups,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HurtboxType {
    Normal,
    Jumping,
    Fallen,
}

#[derive(Component)]
pub struct AttackArcGFX;

#[derive(Component)]
pub struct AttackArc {
    timer: Timer,
    dir: Vec2,
}

impl Hitbox {
    pub fn new(root_entity: Entity, hitbox_type: HitboxType, group: Group, offset: Vec2) -> Self {
        Self {
            root_entity,
            hitbox_type,
            memberships: group,
            offset,
            filters: HURTBOX_GROUP,
        }
    }
}

impl Hurtbox {
    pub fn new(root_entity: Entity, hurtbox_type: HurtboxType, memberships: Group) -> Self {
        Self {
            root_entity,
            hurtbox_type,
            collision_groups: CollisionGroups::new(memberships | HURTBOX_GROUP, HITBOX_GROUP),
        }
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
    let mut hitbox = hitbox.clone();
    hitbox.memberships |= HITBOX_GROUP;
    let transform = Transform::from_translation(hitbox.offset.extend(0.0));
    commands
        .spawn((
            hitbox.clone(),
            collider,
            Sensor,
            CollisionGroups::new(hitbox.memberships, hitbox.filters),
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
    let (collision_groups, collider_color) = if hurtbox.hurtbox_type != HurtboxType::Normal {
        (COLLISION_GROUPS_NONE, COLLIDER_COLOR_BLACK)
    } else {
        (hurtbox.collision_groups, COLLIDER_COLOR_WHITE)
    };
    commands
        .spawn((
            hurtbox,
            collider,
            collider_color,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            collision_groups,
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
    let group = match hitbox_type {
        HitboxType::Player(_) => PLAYER_GROUP,
        HitboxType::Enemy(_) => ENEMY_GROUP,
    };
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
        Hitbox::new(entity, hitbox_type, group, hitbox_offset),
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
            SpatialBundle::from_transform(transform),
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

fn spawn_map_collisions(mut commands: Commands, map_polygon_data: Res<MapPolygonData>) {
    for poly in &map_polygon_data.collider_polygons {
        commands.spawn((
            Collider::convex_hull(poly).expect(
                "polygon should be convertable to convex hull, something went really wrong",
            ),
            ColliderDebugColor(LIME.into()),
            // ShapeBundle {
            //     path: GeometryBuilder::build_as(&shapes::Polygon {
            //         points: poly.clone(),
            //         closed: true,
            //     }),
            //     ..default()
            // },
            // Fill::color(COLLIDER_COLOR.with_alpha(0.5)),
        ));
    }
}

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                disable_attack_arc_hitboxes,
                despawn_attack_arcs,
                spawn_map_collisions.run_if(resource_added::<MapPolygonData>),
            )
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
