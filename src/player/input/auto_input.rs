use bevy::prelude::*;

use super::PlayerInput;

fn move_player(mut player_input: ResMut<PlayerInput>) {
    player_input.move_direction = Vec2::new(1.0, 0.0);
}

pub struct InputTestingPlugin;

impl Plugin for InputTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
