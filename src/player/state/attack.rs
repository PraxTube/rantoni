use std::time::Duration;

use bevy::prelude::*;
use bevy_rancic::prelude::{quat_from_vec2, YSortChild};
use bevy_trickfilm::prelude::*;

use crate::{
    player::Player,
    state::{dude_state_hitbox_frames, Attack, AttackForm, DudeState},
    world::collisions::HitboxDirection,
    GameAssets, GameState,
};

pub struct AttackHandler {
    attack: Attack,
    attack_direction: HitboxDirection,
    chained_attack: AttackForm,
    chainable: bool,
    chain_buffer_timer: Timer,
}

impl Default for AttackHandler {
    fn default() -> Self {
        Self {
            attack: Attack::default(),
            attack_direction: HitboxDirection::Top,
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

    pub fn attack_direction(&self) -> HitboxDirection {
        self.attack_direction
    }

    pub fn set_attack_direction(&mut self, direction: Vec2) {
        self.attack_direction = HitboxDirection::from(direction);
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
    q_players: Query<(Entity, &AnimationPlayer2D, &Player)>,
    mut previous_frame: Local<usize>,
) {
    for (player_entity, animator, player) in &q_players {
        if player.state_machine.state() != DudeState::Attacking {
            continue;
        }

        let (start_frame, _) =
            dude_state_hitbox_frames(player.state_machine.state(), player.state_machine.attack());
        // TODO: This might be an issue if start_frame = 0, also I really don't like this in
        // general but it should work for now.
        if animator.frame() == start_frame && animator.frame() != *previous_frame {
            let mut animator = AnimationPlayer2D::default();
            animator.play(assets.arc_animation.clone());

            let attack = commands
                .spawn((
                    animator,
                    YSortChild(10.0),
                    SpriteBundle {
                        texture: assets.arc.clone(),
                        transform: Transform::from_translation(Vec3::new(-14.0, -14.0, 0.0))
                            .with_rotation(quat_from_vec2(Vec2::new(-1.0, -1.0))),
                        ..default()
                    },
                    TextureAtlas::from(assets.arc_layout.clone()),
                ))
                .id();
            commands.entity(player_entity).add_child(attack);
        }

        *previous_frame = animator.frame();
    }
}

pub struct PlayerAttackStatePlugin;

impl Plugin for PlayerAttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_attack_arcs).run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
