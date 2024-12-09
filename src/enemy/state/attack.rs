use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::events::SpawnHitboxEvent,
    dude::{Attack, DudeState},
    enemy::Enemy,
    world::collisions::{spawn_attack_effect, HitboxType},
    GameAssets, GameState,
};

pub struct AttackHandler {
    attack: Attack,
    timer: Timer,
    attack_direction: Vec2,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack: Attack::default(),
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            attack_direction: Vec2::default(),
        }
    }
}

impl AttackHandler {
    pub fn attack(&self) -> Attack {
        self.attack
    }

    pub fn set_attack(&mut self, attack: Attack, attack_direction: Vec2) {
        self.attack = attack;
        self.attack_direction = attack_direction;
    }

    pub fn attack_timer_finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn tick_attack_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn reset_attack_timer(&mut self) {
        self.timer.reset();
    }

    pub fn attack_direction(&self) -> Vec2 {
        self.attack_direction
    }
}

fn spawn_attack_arcs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_enemies: Query<(Entity, &Enemy)>,
    mut ev_spawn_hitbox: EventReader<SpawnHitboxEvent>,
) {
    for ev in ev_spawn_hitbox.read() {
        let Ok((entity, enemy)) = q_enemies.get(*ev.target) else {
            continue;
        };
        if enemy.state_machine.state() != DudeState::Attacking {
            continue;
        }

        spawn_attack_effect(
            &mut commands,
            &assets,
            entity,
            enemy.state_machine.attack_direction(),
            HitboxType::Enemy(enemy.state_machine.attack()),
        );
    }
}

fn tick_attack_timers(time: Res<Time>, mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.state_machine.tick_attack_timer(time.delta());
    }
}

pub struct EnemyAttackStatePlugin;

impl Plugin for EnemyAttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_attack_arcs, tick_attack_timers).run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
