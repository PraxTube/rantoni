use bevy::prelude::*;
use bevy_rancic::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{dude::Attack, GameAssets, GameState};

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

#[derive(Component)]
pub struct Hurtbox {
    pub root_entity: Entity,
}

#[derive(Component)]
struct AttackArc {
    timer: Timer,
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

impl Default for AttackArc {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
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
    root_entity: Entity,
    offset: Vec2,
    collider: Collider,
    group: Group,
) -> Entity {
    commands
        .spawn((
            Hurtbox { root_entity },
            collider,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::new(HURTBOX_GROUP | group, HITBOX_GROUP),
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
    let (hitbox_offset, collider, pos_offset) = match hitbox_type {
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

    let pos = direction.normalize_or_zero() * pos_offset;
    let transform = if with_rotation {
        Transform::from_translation(pos.extend(0.0)).with_rotation(quat_from_vec2(direction))
    } else {
        Transform::from_translation(pos.extend(0.0))
    };

    let attack_arc = commands
        .spawn((
            AttackArc::default(),
            animator,
            YSortChild(10.0),
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
            TextureAtlas::from(layout),
        ))
        .add_child(hitbox)
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
    q_attack_arcs: Query<(Entity, &AnimationPlayer2D), With<AttackArc>>,
) {
    for (entity, animator) in &q_attack_arcs {
        if animator.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct WorldCollisionPlugin;

impl Plugin for WorldCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (disable_attack_arc_hitboxes, despawn_attack_arcs)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
