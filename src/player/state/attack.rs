use std::time::Duration;

use bevy::prelude::*;
use bevy_rancic::prelude::{
    quat_from_vec2, YSortChild, COLLIDER_COLOR_BLACK, COLLISION_GROUPS_NONE,
};
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    player::Player,
    state::{dude_state_hitbox_start_frame, Attack, AttackForm, DudeState},
    world::collisions::{spawn_hitbox_collision, Hitbox, HitboxType, PLAYER_GROUP},
    GameAssets, GameState,
};

const ARC_OFFSET: f32 = 20.0;

pub struct AttackHandler {
    attack: Attack,
    attack_direction: Vec2,
    chained_attack: AttackForm,
    chainable: bool,
    chain_buffer_timer: Timer,
}

#[derive(Component)]
struct AttackArc {
    timer: Timer,
}

impl Default for AttackArc {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack: Attack::default(),
            attack_direction: Vec2::default(),
            chained_attack: AttackForm::default(),
            chainable: false,
            chain_buffer_timer: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

impl AttackHandler {
    pub fn attack(&self) -> Attack {
        self.attack
    }

    pub fn set_attack(&mut self, attack: Attack) {
        self.attack = attack;
    }

    pub fn attack_direction(&self) -> Vec2 {
        self.attack_direction
    }

    pub fn set_attack_direction(&mut self, direction: Vec2) {
        self.attack_direction = direction;
    }

    pub fn chained_attack(&self) -> AttackForm {
        self.chained_attack
    }

    pub fn set_chained_attack(&mut self, chained_attack: AttackForm) {
        self.chained_attack = chained_attack;
    }

    pub fn chainable(&self) -> bool {
        self.chainable
    }

    pub fn set_chainable(&mut self, chainable: bool) {
        self.chainable = chainable;
        self.chain_buffer_timer.pause();
    }

    pub fn start_attack_chain_timer(&mut self) {
        self.chain_buffer_timer.unpause();
        self.chain_buffer_timer.reset();
    }

    pub fn handle_attack_chain_timer(&mut self, delta: Duration) {
        self.chain_buffer_timer.tick(delta);
        if self.chain_buffer_timer.just_finished() {
            self.chainable = false;
        }
    }
}

fn spawn_attack_arc(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    player_entity: Entity,
    direction: Vec2,
) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.arc_animation.clone());

    let pos = direction.normalize_or_zero() * ARC_OFFSET;
    let hitbox = spawn_hitbox_collision(
        commands,
        Hitbox::new(
            player_entity,
            HitboxType::Player(Attack::Light1),
            PLAYER_GROUP,
            Vec2::ZERO,
        ),
        Collider::cuboid(8.0, 14.0),
    );

    let attack_arc = commands
        .spawn((
            AttackArc::default(),
            animator,
            YSortChild(10.0),
            SpriteBundle {
                texture: assets.arc.clone(),
                transform: Transform::from_translation(pos.extend(0.0))
                    .with_rotation(quat_from_vec2(direction)),
                ..default()
            },
            TextureAtlas::from(assets.arc_layout.clone()),
        ))
        .add_child(hitbox)
        .id();
    commands.entity(player_entity).add_child(attack_arc);
}

fn spawn_attack_arcs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_players: Query<(Entity, &AnimationPlayer2D, &Player)>,
    mut previous_frame: Local<usize>,
) {
    for (player_entity, animator, player) in &q_players {
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        let start_frame = dude_state_hitbox_start_frame(
            player.state_machine.state(),
            player.state_machine.attack(),
        );
        // TODO: This might be an issue if start_frame = 0, also I really don't like this in
        // general but it should work for now.
        if animator.frame() == start_frame && animator.frame() != *previous_frame {
            spawn_attack_arc(
                &mut commands,
                &assets,
                player_entity,
                player.state_machine.attack_direction(),
            );
        }

        *previous_frame = animator.frame();
    }
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

pub struct PlayerAttackStatePlugin;

impl Plugin for PlayerAttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_attack_arcs,
                disable_attack_arc_hitboxes,
                despawn_attack_arcs,
            )
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
