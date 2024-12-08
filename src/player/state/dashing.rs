use bevy::prelude::*;

use crate::{
    dude::{dude_dashing_sprites, DudeState},
    player::Player,
    GameAssets, GameState,
};

use super::PlayerStateSystemSet;

const DASH_TIME: f32 = 0.25;
/// The number of frames one dash animation contains. This number shouldn't change.
const DASH_FRAME_COUNT: usize = 2;

#[derive(Component)]
struct DashSprite {
    timer: Timer,
}

impl Default for DashSprite {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.25, TimerMode::Once),
        }
    }
}

pub struct DashingTimer(pub Timer);

impl Default for DashingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(DASH_TIME, TimerMode::Repeating))
    }
}

fn tick_timers(time: Res<Time>, mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.state() == DudeState::Dashing {
            player.state_machine.tick_dashing_timer(time.delta());
        }
    }
}

fn spawn_dash_sprites(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_players: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &q_players {
        if player.state_machine.state() != DudeState::Dashing {
            continue;
        }

        let (texture, direction_index_offset) =
            dude_dashing_sprites(&assets, player.state_machine.attack_direction());
        let layout = assets.dude_layout.clone();

        let index = DASH_FRAME_COUNT * direction_index_offset;

        commands.spawn((
            DashSprite::default(),
            SpriteBundle {
                transform: Transform::from_translation(transform.translation),
                texture,
                ..default()
            },
            TextureAtlas { layout, index },
        ));
    }
}

fn fade_dash_sprites(time: Res<Time>, mut q_dash_sprites: Query<(&mut Sprite, &mut DashSprite)>) {
    for (mut sprite, mut dash_sprite) in &mut q_dash_sprites {
        let t = dash_sprite.timer.elapsed_secs() / dash_sprite.timer.duration().as_secs_f32();
        sprite.color.set_alpha((1.0 - t).max(0.0));
        dash_sprite.timer.tick(time.delta());
    }
}

fn despawn_dash_sprites(mut commands: Commands, q_dash_sprites: Query<(Entity, &DashSprite)>) {
    for (entity, dash_sprite) in &q_dash_sprites {
        if dash_sprite.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_player_dash_directions(mut q_players: Query<&mut Player>) {
    for mut player in &mut q_players {
        if player.state_machine.state() == DudeState::Dashing {
            // TODO: Do you want to allow this? It feels pretty nice, though hades doesn't allow you to
            // change your dash direction mid-air, but I feel like it's kinda nice.
            let direction = player.current_direction;
            player.state_machine.set_attack_direction(direction);
        }
    }
}

pub struct PlayerDashingPlugin;

impl Plugin for PlayerDashingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_timers,
                spawn_dash_sprites,
                fade_dash_sprites,
                despawn_dash_sprites,
                update_player_dash_directions,
            )
                .before(PlayerStateSystemSet)
                .run_if(not(in_state(GameState::AssetLoading))),
        );
    }
}
