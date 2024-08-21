use bevy::{ecs::system::SystemId, prelude::*};
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

use super::{input::PlayerInput, Player};

#[derive(Event)]
pub struct PlayerChangedState {
    state: PlayerState,
    previous_state: PlayerState,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerAttackState {
    #[default]
    Light1,
    Light2,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ChainAttack {
    #[default]
    None,
    Light,
    Heavy,
}

#[derive(Component)]
pub struct PlayerStateMachine {
    state: PlayerState,
    attack_state: PlayerAttackState,
    previous_state: PlayerState,
    just_changed_state: bool,
    chain_attack: ChainAttack,
    update_animation: SystemId,
}

impl FromWorld for PlayerStateMachine {
    fn from_world(world: &mut World) -> Self {
        let update_animation = world.register_system(update_player_animation);
        Self {
            state: PlayerState::default(),
            attack_state: PlayerAttackState::default(),
            previous_state: PlayerState::default(),
            just_changed_state: false,
            chain_attack: ChainAttack::default(),
            update_animation,
        }
    }
}

impl PlayerStateMachine {
    pub fn can_run(&self) -> bool {
        self.state == PlayerState::Idling
            || self.state == PlayerState::Running
            || self.state == PlayerState::Recovering
    }

    fn can_punch(&self) -> bool {
        self.can_run()
            || self.state == PlayerState::Attacking
                && self.attack_state == PlayerAttackState::Light1
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }

    pub fn set_state(&mut self, commands: &mut Commands, state: PlayerState) {
        self.previous_state = self.state;
        self.just_changed_state = true;
        self.state = state;
        commands.run_system(self.update_animation);
    }

    pub fn attack_state(&self) -> PlayerAttackState {
        self.attack_state
    }

    pub fn attack_state_eq(&self, attack_state: PlayerAttackState) -> bool {
        self.state == PlayerState::Attacking && self.attack_state == attack_state
    }

    pub fn set_attack_state(&mut self, commands: &mut Commands, attack_state: PlayerAttackState) {
        self.set_state(commands, PlayerState::Attacking);
        self.attack_state = attack_state;
    }

    pub fn chain_attack(&self) -> ChainAttack {
        self.chain_attack
    }

    pub fn set_chain_attack(&mut self, chain_attack: ChainAttack) {
        self.chain_attack = chain_attack;
    }

    fn state_animation(&self, assets: &Res<GameAssets>) -> (Handle<AnimationClip2D>, bool) {
        match self.state {
            PlayerState::Idling => (assets.player_animations[0].clone(), true),
            PlayerState::Running => (assets.player_animations[1].clone(), true),
            PlayerState::Attacking => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[2].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[4].clone(), false),
            },
            PlayerState::Recovering => match self.attack_state {
                PlayerAttackState::Light1 => (assets.player_animations[3].clone(), false),
                PlayerAttackState::Light2 => (assets.player_animations[5].clone(), false),
            },
        }
    }
}

fn update_player_animation(
    assets: Res<GameAssets>,
    mut q_player: Query<(&Player, &mut AnimationPlayer2D)>,
) {
    let (player, mut animator) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (animation, repeat) = player.state_machine.state_animation(&assets);
    if repeat {
        animator.play(animation).repeat();
    } else {
        animator.play(animation);
    }
}

fn transition_run_state(
    mut commands: Commands,
    player_input: Res<PlayerInput>,
    mut q_player: Query<&mut Player>,
) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if !player.state_machine.can_run() {
        return;
    }

    if player_input.move_direction != Vec2::ZERO {
        if player.state_machine.state() != PlayerState::Running {
            player
                .state_machine
                .set_state(&mut commands, PlayerState::Running);
        }
    } else if player.state_machine.state() == PlayerState::Running {
        player
            .state_machine
            .set_state(&mut commands, PlayerState::Idling);
    };
}

fn transition_punch_state(
    mut commands: Commands,
    player_input: Res<PlayerInput>,
    mut q_player: Query<&mut Player>,
) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if !player.state_machine.can_punch() {
        return;
    }

    if player_input.punched {
        player.punching_direction = player_input.aim_direction;
        if player
            .state_machine
            .attack_state_eq(PlayerAttackState::Light1)
        {
            player.state_machine.set_chain_attack(ChainAttack::Light);
        } else {
            info!(
                "punching, state: {:?}, a_state: {:?}",
                player.state_machine.state(),
                player.state_machine.attack_state()
            );
            player
                .state_machine
                .set_attack_state(&mut commands, PlayerAttackState::Light1);
            info!(
                "new state: {:?}, a_state: {:?}",
                player.state_machine.state(),
                player.state_machine.attack_state()
            );
        }
    }
}

fn transition_idle_state(
    mut commands: Commands,
    mut q_player: Query<(&mut Player, &AnimationPlayer2D)>,
) {
    let Ok((mut player, animator)) = q_player.get_single_mut() else {
        return;
    };

    if animator.just_finished() {
        warn!("fin, state: {:?}", player.state_machine.state());
        match player.state_machine.state() {
            PlayerState::Idling => error!("should never happen! Idle should be repeating forever"),
            PlayerState::Running => {
                error!("should never happen! Running should be repeating forever")
            }
            PlayerState::Attacking => {
                if player.state_machine.chain_attack() == ChainAttack::None {
                    player
                        .state_machine
                        .set_state(&mut commands, PlayerState::Recovering);
                } else {
                    player.state_machine.set_chain_attack(ChainAttack::None);
                    match player.state_machine.attack_state() {
                        PlayerAttackState::Light1 => player
                            .state_machine
                            .set_attack_state(&mut commands, PlayerAttackState::Light2),
                        PlayerAttackState::Light2 => player
                            .state_machine
                            .set_state(&mut commands, PlayerState::Recovering),
                    };
                }
            }
            PlayerState::Recovering => {
                player
                    .state_machine
                    .set_state(&mut commands, PlayerState::Idling);
            }
        };
    }
}

pub fn trigger_player_changed_state(
    mut q_player: Query<&mut Player>,
    mut ev_player_changed_state: EventWriter<PlayerChangedState>,
) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    if player.state_machine.just_changed_state {
        player.state_machine.just_changed_state = false;
        ev_player_changed_state.send(PlayerChangedState {
            state: player.state_machine.state,
            previous_state: player.state_machine.previous_state,
        });
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerChangedState>().add_systems(
            Update,
            (
                transition_idle_state,
                transition_run_state,
                transition_punch_state,
                trigger_player_changed_state,
            )
                .chain(),
        );
    }
}
