mod attack;
mod dashing;
mod jumping;
mod state_machine;

pub use attack::AttackHandler;
pub use state_machine::PlayerStateMachine;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::dude::{AttackForm, DudeState, Health, ParryState};
use crate::player::{input::PlayerInput, Player};

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            attack::PlayerAttackStatePlugin,
            jumping::PlayerJumpingStatePlugin,
            dashing::PlayerDashingPlugin,
        ))
        .add_systems(PreUpdate, reset_just_changed)
        .add_systems(
            Update,
            (
                transition_dying_state,
                transition_stagger_state,
                transition_parry_state,
                transition_dash_state,
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

fn transition_dying_state(mut q_players: Query<(&mut Player, &Health)>) {
    for (mut player, health) in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }

        if health.health == 0 {
            player.state_machine.set_state(DudeState::Dying);
        }
    }
}

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
        if !player.state_machine.can_attack() {
            continue;
        }

        player
            .state_machine
            .set_state(DudeState::Parrying(ParryState::Start));
    }
}

fn transition_dash_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        if !player_input.dash {
            continue;
        }
        if player.state_machine.state() == DudeState::Staggering {
            continue;
        }

        player.state_machine.set_state(DudeState::Dashing);
    }
}

fn transition_attacking_state(player_input: Res<PlayerInput>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.just_changed() {
            continue;
        }
        if !player.state_machine.can_attack() {
            continue;
        }

        // TODO: You would have to actually figure out which controls belong to which player in local
        // multiplayer
        let attack_form = if player_input.light_attack {
            AttackForm::Light
        } else if player_input.heavy_attack {
            AttackForm::Heavy
        } else if player_input.special_light {
            AttackForm::SpecialLight
        } else if player_input.special_heavy {
            AttackForm::SpecialHeavy
        } else {
            continue;
        };

        let attack_direction = if player_input.aim_direction != Vec2::ZERO {
            player_input.aim_direction
        } else {
            player.current_direction
        };
        player.state_machine.set_attack_direction(attack_direction);
        player.current_direction = attack_direction;

        player.state_machine.transition_attack(attack_form);
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
            DudeState::Idling | DudeState::Running | DudeState::Dying => {}
            DudeState::Stalking => error!("player must never be in stalking state, you should refactor this in the first place"),
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
            DudeState::Dashing => {
                if player.state_machine.dashing_just_finished() {
                    player.state_machine.set_state(DudeState::Idling);
                }
            }
            DudeState::Parrying(parry_state) => {
                if !animator.just_finished() {
                    continue;
                }
                match parry_state {
                    ParryState::Start => player
                        .state_machine
                        .set_state(DudeState::Parrying(ParryState::Fail)),
                    ParryState::Success | ParryState::Fail => {
                        player.state_machine.set_state(DudeState::Idling)
                    }
                }
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
