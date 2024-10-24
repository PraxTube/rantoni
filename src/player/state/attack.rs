use std::time::Duration;

use bevy::prelude::*;

use crate::{
    assets::events::SpawnHitboxEvent,
    dude::{Attack, AttackForm, DudeState},
    player::Player,
    world::collisions::{spawn_attack_effect, HitboxType},
    GameAssets, GameState,
};

pub struct AttackHandler {
    attack: Attack,
    attack_direction: Vec2,
    chained_attack: AttackForm,
    chainable: bool,
    chain_buffer_timer: Timer,
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

pub struct PlayerAttackStatePlugin;

impl Plugin for PlayerAttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_attack_arcs,).run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
