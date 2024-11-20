use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_tweening::{lens::*, *};

use crate::GameState;

#[derive(Component)]
struct ScreenFader;

#[derive(Event)]
pub struct FadeScreen {
    duration: f32,
    start: Color,
    end: Color,
    ease_function: EaseFunction,
    event: u64,
}

impl FadeScreen {
    pub fn new(
        duration: f32,
        start: Color,
        end: Color,
        ease_function: EaseFunction,
        event: u64,
    ) -> Self {
        Self {
            duration,
            start,
            end,
            ease_function,
            event,
        }
    }
}

fn spawn_screen_fader(mut commands: Commands) {
    commands.spawn((
        ScreenFader,
        ImageBundle {
            style: Style {
                // Cover a little more than the entire screen.
                width: Val::Vw(110.0),
                height: Val::Vh(110.0),
                ..default()
            },
            background_color: BackgroundColor(BLACK.into()),
            z_index: ZIndex::Global(999),
            ..default()
        },
    ));
}

fn tween_screen_fader(
    mut commands: Commands,
    q_screen_fader: Query<Entity, With<ScreenFader>>,
    mut ev_spawn_fade_screen: EventReader<FadeScreen>,
) {
    let Ok(entity) = q_screen_fader.get_single() else {
        return;
    };

    for ev in ev_spawn_fade_screen.read() {
        let tween = Tween::new(
            ev.ease_function,
            std::time::Duration::from_secs_f32(ev.duration),
            UiBackgroundColorLens {
                start: ev.start,
                end: ev.end,
            },
        )
        .with_completed_event(ev.event);

        commands.entity(entity).insert(Animator::new(tween));
    }
}

pub struct ScreenFadeUiPlugin;

impl Plugin for ScreenFadeUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FadeScreen>()
            .add_systems(OnEnter(GameState::AssetLoading), spawn_screen_fader)
            .add_systems(Update, tween_screen_fader);
    }
}
