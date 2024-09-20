use std::time::Duration;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, Player};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum PlayerAttackState {
    #[default]
    Light1,
    Light2,
    Light3,
    Heavy1,
    Heavy2,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Attack {
    #[default]
    None,
    Light,
    Heavy,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerStateSystemSet;

#[derive(Component, Default)]
pub struct PlayerStateMachine {
    just_changed: bool,
    state: PlayerState,
    previous_state: PlayerState,
    attack_handler: AttackHandler,
}

struct AttackHandler {
    attack_state: PlayerAttackState,
    chained_attack: Attack,
    chainable: bool,
    chain_buffer_timer: Timer,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack_state: PlayerAttackState::default(),
            chained_attack: Attack::default(),
            chainable: false,
            chain_buffer_timer: Timer::from_seconds(0.3, TimerMode::Once),
        }
    }
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == PlayerState::Idling
            || self.state == PlayerState::Running
            || self.state == PlayerState::Recovering
    }

    pub fn can_punch(&self) -> bool {
        self.can_run() || self.state == PlayerState::Attacking
    }

    fn previous_state(&self) -> PlayerState {
        self.previous_state
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    fn set_state(&mut self, state: PlayerState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }

        self.set_just_changed(true);
        self.previous_state = self.state;
        self.state = state;
    }

    pub fn attack_state(&self) -> PlayerAttackState {
        self.attack_handler.attack_state
    }

    pub fn attack_state_eq(&self, attack_state: PlayerAttackState) -> bool {
        self.state == PlayerState::Attacking && self.attack_state() == attack_state
    }

    fn set_attack_state(&mut self, attack_state: PlayerAttackState) {
        if self.just_changed {
            error!("Trying to set state even though it was already changed this frame. Should never happen, you probably forgot a flag check");
            return;
        }
        self.set_state(PlayerState::Attacking);
        self.attack_handler.attack_state = attack_state;
        self.attack_handler.chainable = true;
        self.attack_handler.chain_buffer_timer.pause();
    }

    pub fn chained_attack(&self) -> Attack {
        self.attack_handler.chained_attack
    }

    fn set_chained_attack(&mut self, attack: Attack) {
        self.attack_handler.chained_attack = attack;
    }

    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    fn set_just_changed(&mut self, just_changed: bool) {
        self.just_changed = just_changed;
    }

    fn start_attack_chain_timer(&mut self) {
        self.attack_handler.chain_buffer_timer.unpause();
        self.attack_handler.chain_buffer_timer.reset();
    }

    fn handle_attack_chain_timer(&mut self, delta: Duration) {
        self.attack_handler.chain_buffer_timer.tick(delta);
        if self.attack_handler.chain_buffer_timer.just_finished() {
            self.attack_handler.chainable = false;
        }
    }

    fn default_attack(&mut self, attack: Attack) {
        match attack {
            Attack::None => {}
            Attack::Light => self.set_attack_state(PlayerAttackState::Light1),
            Attack::Heavy => self.set_attack_state(PlayerAttackState::Heavy1),
        }
    }

    fn combo_attack(&self, attack: Attack) -> Option<PlayerAttackState> {
        match self.attack_state() {
            PlayerAttackState::Light1 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light2),
                Attack::Heavy => Some(PlayerAttackState::Heavy1),
            },
            PlayerAttackState::Light2 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light3),
                Attack::Heavy => None,
            },
            PlayerAttackState::Light3 => None,
            PlayerAttackState::Heavy1 => match attack {
                Attack::None => panic!("should never happen!"),
                Attack::Light => Some(PlayerAttackState::Light2),
                Attack::Heavy => Some(PlayerAttackState::Heavy2),
            },
            PlayerAttackState::Heavy2 => None,
        }
    }

    fn transition_chain_attack(&mut self) {
        if self.chained_attack() == Attack::None {
            self.set_state(PlayerState::Recovering);
            return;
        }

        match self.combo_attack(self.chained_attack()) {
            Some(attack_state) => self.set_attack_state(attack_state),
            None => self.set_state(PlayerState::Recovering),
        }
        self.set_chained_attack(Attack::None);
    }

    fn transition_attack(&mut self, attack: Attack) {
        if self.just_changed() {
            return;
        }
        if !self.can_punch() {
            return;
        }

        if self.state() == PlayerState::Attacking {
            assert_ne!(attack, Attack::None);
            self.set_chained_attack(attack);
        } else if self.attack_handler.chainable {
            match self.combo_attack(attack) {
                Some(attack_state) => self.set_attack_state(attack_state),
                None => self.default_attack(attack),
            }
        } else {
            self.default_attack(attack);
        }
    }

    pub fn state_animation(&self, assets: &Res<GameAssets>) -> (Handle<AnimationClip2D>, bool) {
        match self.state {
            PlayerState::Idling => (assets.player_animations[0].clone(), true),
            PlayerState::Running => (assets.player_animations[1].clone(), true),
            PlayerState::Attacking => match self.attack_state() {
                PlayerAttackState::Light1 => (assets.player_animations[2].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[4].clone(), false),
                PlayerAttackState::Light3 => (assets.player_animations[11].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[7].clone(), false),
                PlayerAttackState::Heavy2 => (assets.player_animations[9].clone(), false),
            },
            PlayerState::Recovering => match self.attack_state() {
                PlayerAttackState::Light1 => (assets.player_animations[3].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[5].clone(), false),
                PlayerAttackState::Light3 => (assets.player_animations[12].clone(), false),
                PlayerAttackState::Heavy1 => (assets.player_animations[8].clone(), false),
                PlayerAttackState::Heavy2 => (assets.player_animations[10].clone(), false),
            },
        }
    }

    pub fn state_hitbox_frames(&self) -> (usize, usize) {
        match self.state {
            PlayerState::Idling => {
                error!("should never happen! idle doesn't have any hitbox frames");
                (0, 0)
            }
            PlayerState::Running => {
                error!("should never happen! run doesn't have any hitbox frames");
                (0, 0)
            }
            PlayerState::Attacking => match self.attack_state() {
                PlayerAttackState::Light1 => (0, 1),
                PlayerAttackState::Light2 => (0, 1),
                PlayerAttackState::Light3 => (1, 2),
                PlayerAttackState::Heavy1 => (1, 2),
                PlayerAttackState::Heavy2 => (1, 2),
            },
            PlayerState::Recovering => {
                error!("should never happen! recover doesn't have any hitbox frames");
                (0, 0)
            }
        }
    }
}

fn transition_attack_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        let attack = if player_input.light_attack {
            Attack::Light
        } else if player_input.heavy_attack {
            Attack::Heavy
        } else {
            continue;
        };
        player.state_machine.transition_attack(attack);
    }
}

fn transition_run_state(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };
    if player.state_machine.just_changed() {
        return;
    }

    if !player.state_machine.can_run() {
        return;
    }

    if player_input.move_direction != Vec2::ZERO {
        if player.state_machine.state() != PlayerState::Running {
            player.state_machine.set_state(PlayerState::Running);
        }
    } else if player.state_machine.state() == PlayerState::Running {
        player.state_machine.set_state(PlayerState::Idling);
    };
}

fn transition_idle_state(mut q_player: Query<(&mut Player, &AnimationPlayer2D)>) {
    let Ok((mut player, animator)) = q_player.get_single_mut() else {
        return;
    };
    if player.state_machine.just_changed() {
        return;
    }

    if !animator.just_finished() {
        return;
    }

    match player.state_machine.state() {
        PlayerState::Idling | PlayerState::Running => {
            error!("should never happen! The current state's animation should be repeating forever and never finish")
        }
        PlayerState::Attacking => {
            player.state_machine.transition_chain_attack();
        }
        PlayerState::Recovering => {
            player.state_machine.set_state(PlayerState::Idling);
        }
    };
}

fn reset_just_changed(mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    player.state_machine.set_just_changed(false);
}

fn update_aim_direction(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player_input.aim_direction == Vec2::ZERO {
            continue;
        }

        if player.state_machine.just_changed() {
            player.aim_direction = player_input.aim_direction;
        }
    }
}

fn start_attack_chain_timer(mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if !player.state_machine.just_changed() {
            continue;
        }

        if player.state_machine.previous_state() == PlayerState::Attacking
            && player.state_machine.state() != PlayerState::Attacking
        {
            player.state_machine.start_attack_chain_timer();
        }
    }
}

fn handle_attack_chain_timer(time: Res<Time>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        player.state_machine.handle_attack_chain_timer(time.delta());
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_just_changed,
                transition_attack_state,
                transition_idle_state,
                transition_run_state,
            )
                .chain()
                .in_set(PlayerStateSystemSet),
        )
        .add_systems(
            Update,
            (
                update_aim_direction,
                start_attack_chain_timer,
                handle_attack_chain_timer,
            )
                .chain()
                .after(PlayerStateSystemSet),
        );
    }
}
