use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::plugin::RapierTransformPropagateSet;

use crate::{
    assets::{events::SpawnHitboxEvent, PLAYER_SPRITE_SIZE},
    dude::{Attack, AttackForm, DudeState},
    player::Player,
    world::collisions::{spawn_attack_effect, AttackArc, AttackArcGFX, HitboxType},
    GameAssets, GameState,
};

pub struct AttackHandler {
    attack: Attack,
    attack_direction: Vec2,
    chained_attack: AttackForm,
    chainable: bool,
    chain_buffer_timer: Timer,
    // TODO: Only temporary, a proper animation curve implementation would be better.
    // Also only used for normal attacks (not stuff like dropkick etc).
    can_move: bool,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack: Attack::default(),
            attack_direction: Vec2::default(),
            chained_attack: AttackForm::default(),
            chainable: false,
            chain_buffer_timer: Timer::from_seconds(0.3, TimerMode::Once),
            can_move: true,
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

    pub fn can_move(&self) -> bool {
        self.can_move
    }

    pub fn set_can_move(&mut self, can_move: bool) {
        self.can_move = can_move;
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

fn spawn_attack_arcs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_players: Query<(Entity, &Player)>,
    mut ev_spawn_hitbox: EventReader<SpawnHitboxEvent>,
) {
    for ev in ev_spawn_hitbox.read() {
        let Ok((entity, player)) = q_players.get(*ev.target) else {
            continue;
        };
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        spawn_attack_effect(
            &mut commands,
            &assets,
            entity,
            player.state_machine.attack_direction(),
            HitboxType::Player(player.state_machine.attack()),
        );
    }
}

fn move_attack_arcs_jumping(
    q_players: Query<(&Children, &Sprite), With<Player>>,
    q_attack_arcs: Query<(&Children, &AttackArc), Without<Player>>,
    mut q_attack_arc_gfxs: Query<&mut Transform, With<AttackArcGFX>>,
) {
    for (player_children, sprite) in &q_players {
        let anchor = match sprite.anchor {
            // Only jumping attacks have this Custom anchor, that is how we filter for them.
            Anchor::Custom(v) => Vec2::new(v.x, -v.y) * PLAYER_SPRITE_SIZE as f32,
            _ => continue,
        };

        for child in player_children {
            let Ok((children, attack_arc)) = q_attack_arcs.get(*child) else {
                continue;
            };

            for child in children {
                let Ok(mut transform) = q_attack_arc_gfxs.get_mut(*child) else {
                    continue;
                };

                let dir = anchor_rotation_direction(attack_arc.dir());
                transform.translation = anchor.rotate(dir).extend(0.0);
            }
        }
    }
}

fn anchor_rotation_direction(dir: Vec2) -> Vec2 {
    let angle = dir.angle_between(Vec2::Y);

    if angle.abs() < PI / 8.0 {
        // Top
        -dir
    } else if angle.abs() < 3.0 / 8.0 * PI {
        // Diagonal Up
        if angle > 0.0 {
            // Top Right
            Vec2::new(dir.y, -dir.x)
        } else {
            // Top Left
            Vec2::new(-dir.y, dir.x)
        }
    } else if angle.abs() < 5.0 / 8.0 * PI {
        // Side
        dir
    } else if angle.abs() < 7.0 / 8.0 * PI {
        // Diagonal Down
        if angle > 0.0 {
            // Bottom Right
            Vec2::new(-dir.y, dir.x)
        } else {
            // Bottom Left
            Vec2::new(dir.y, -dir.x)
        }
    } else {
        // Bottom
        -dir
    }
}

pub struct PlayerAttackStatePlugin;

impl Plugin for PlayerAttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_attack_arcs,).run_if(not(in_state(GameState::AssetLoading))),
        )
        .add_systems(
            PostUpdate,
            move_attack_arcs_jumping.before(RapierTransformPropagateSet),
        );
    }
}
