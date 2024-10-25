mod attack;
mod jumping;
mod state_machine;

pub use attack::AttackHandler;
pub use state_machine::PlayerStateMachine;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::dude::{Attack, AttackForm, DudeState, JumpingState, ParryState};
use crate::player::{input::PlayerInput, Player};

const SLIDING_TO_JUMP_TRANSITION_MAX_TIME_PERCENTAGE: f32 = 0.7;
const JUMP_TO_SLIDING_TRANSITION_MIN_TIME_PERCENTAGE: f32 = 0.6;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            attack::PlayerAttackStatePlugin,
            jumping::PlayerJumpingStatePlugin,
        ))
        .add_systems(PreUpdate, reset_just_changed)
        .add_systems(
            Update,
            (
                transition_stagger_state,
                transition_parry_state,
                transition_slide_state,
                transition_jump_state,
                transition_attacking_state,
                transition_idle_state,
                transition_run_state,
                reset_new_state,
            )
                .chain()
                .in_set(PlayerStateSystemSet),
        )
        .add_systems(
            Update,
            (start_attack_chain_timer, handle_attack_chain_timer)
                .chain()
                .after(PlayerStateSystemSet),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerStateSystemSet;

fn transition_stagger_state(mut q_players: Query<(&mut AnimationPlayer2D, &mut Player)>) {
    for (mut animator, mut player) in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        let Some(new_state) = player.state_machine.new_state() else {
            continue;
        };
        if new_state != DudeState::Staggering {
            continue;
        }

        if player.state_machine.state() == DudeState::Staggering {
            animator.replay();
        } else {
            player.state_machine.set_state(DudeState::Staggering);
        }
    }
}

fn transition_parry_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        if !player_input.parry {
            continue;
        }
        if !player.state_machine.can_run() {
            continue;
        }

        player.state_machine.set_parry_state(ParryState::Start);
    }
}

fn transition_slide_state(
    player_input: Res<PlayerInput>,
    mut q_players: Query<(&AnimationPlayer2D, &mut Player)>,
) {
    for (animator, mut player) in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        if !player_input.slide {
            continue;
        }
        let x = animator.elapsed() / animator.duration().unwrap_or(1000.0);
        if !(player.state_machine.can_attack()
            || (player.state_machine.state() == DudeState::Jumping(JumpingState::Start)
                && x >= JUMP_TO_SLIDING_TRANSITION_MIN_TIME_PERCENTAGE))
        {
            continue;
        }

        player.state_machine.set_sliding_attack();
    }
}

fn transition_jump_state(
    player_input: Res<PlayerInput>,
    mut q_players: Query<(&AnimationPlayer2D, &mut Player)>,
) {
    for (animator, mut player) in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        if !player_input.jump {
            continue;
        }
        let x = animator.elapsed() / animator.duration().unwrap_or(1000.0);
        if !(player.state_machine.can_run()
            || (player.state_machine.attack_eq(Attack::Slide)
                && x <= SLIDING_TO_JUMP_TRANSITION_MAX_TIME_PERCENTAGE))
        {
            continue;
        }
        if let DudeState::Jumping(_) = player.state_machine.state() {
            continue;
        }

        player
            .state_machine
            .set_state(DudeState::Jumping(JumpingState::Start));
    }
}

fn transition_attacking_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        // TODO: You would have to actually figure out which controls belong to which player in local
        // multiplayer
        let attack_form = if player_input.light_attack {
            AttackForm::Light
        } else if player_input.heavy_attack {
            AttackForm::Heavy
        } else {
            continue;
        };

        if player.state_machine.state() == DudeState::Jumping(JumpingState::Start) {
            match attack_form {
                AttackForm::None => {}
                AttackForm::Light => todo!(),
                AttackForm::Heavy => player.state_machine.set_attack(Attack::Dropkick),
            };
        } else {
            player.state_machine.transition_attack(attack_form);
        }
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
        if player.state_machine.state() != DudeState::Running {
            player.state_machine.set_state(DudeState::Running);
        }
    } else if player.state_machine.state() == DudeState::Running {
        player.state_machine.set_state(DudeState::Idling);
    };
}

fn transition_idle_state(
    player_input: Res<PlayerInput>,
    mut q_players: Query<(&AnimationPlayer2D, &mut Player)>,
) {
    for (animator, mut player) in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }

        match player.state_machine.state() {
            DudeState::Idling | DudeState::Running => {}
            DudeState::Attacking => {
                if !animator.just_finished() {
                    continue;
                }
                player
                    .state_machine
                    .transition_chain_attack(player_input.move_direction);
            }
            DudeState::Recovering => {
                if animator.just_finished() {
                    player.state_machine.set_state(DudeState::Idling);
                }
            }
            DudeState::Staggering => {
                if player.state_machine.stagger_state().is_recovering() {
                    if animator.just_finished() {
                        player.state_machine.set_state(DudeState::Idling);
                    }
                } else if player.state_machine.stagger_just_finished() {
                    player.state_machine.set_stagger_state_recover();
                }
            }
            DudeState::Parrying => {
                if !animator.just_finished() {
                    continue;
                }
                match player.state_machine.parry_state() {
                    ParryState::Start => player.state_machine.set_parry_state(ParryState::Fail),
                    ParryState::Success => {
                        player.state_machine.set_parry_state(ParryState::Recover)
                    }
                    ParryState::Recover | ParryState::Fail => {
                        player.state_machine.set_state(DudeState::Idling)
                    }
                }
            }
            DudeState::Jumping(jumping_state) => {
                if !animator.just_finished() {
                    continue;
                }
                let new_state = match jumping_state {
                    JumpingState::Start => {
                        if player_input.move_direction == Vec2::ZERO {
                            DudeState::Jumping(JumpingState::RecoverIdle)
                        } else {
                            DudeState::Jumping(JumpingState::RecoverMoving)
                        }
                    }
                    JumpingState::RecoverIdle => DudeState::Idling,
                    JumpingState::RecoverMoving => DudeState::Running,
                };
                player.state_machine.set_state(new_state);
            }
        };
    }
}

fn reset_just_changed(mut q_player: Query<&mut Player>) {
    let Ok(mut player) = q_player.get_single_mut() else {
        return;
    };

    player.state_machine.set_just_changed(false);
}

fn reset_new_state(mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        player.state_machine.reset_new_state();
    }
}

fn start_attack_chain_timer(mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if !player.state_machine.just_changed() {
            continue;
        }

        if player.state_machine.previous_state() == DudeState::Attacking
            && player.state_machine.state() != DudeState::Attacking
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
