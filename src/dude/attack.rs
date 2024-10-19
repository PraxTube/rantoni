use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameAssets;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Attack {
    #[default]
    Light1,
    Light2,
    Light3,
    Heavy1,
    Heavy2,
    Heavy3,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AttackForm {
    #[default]
    None,
    Light,
    Heavy,
}

impl Attack {
    /// Return the
    ///     - Hitbox offset
    ///     - Collider of hitbox
    ///     - Magnitude of the offset to the parent entity
    /// of the current attack.
    pub fn effect_position_data(&self) -> (Vec2, Collider, f32) {
        match self {
            Attack::Light1 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
            Attack::Light2 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
            Attack::Light3 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
            Attack::Heavy1 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
            Attack::Heavy2 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
            Attack::Heavy3 => (Vec2::default(), Collider::cuboid(8.0, 14.0), 20.0),
        }
    }

    pub fn effect_animation_data(
        &self,
        assets: &Res<GameAssets>,
    ) -> (
        Handle<Image>,
        Handle<TextureAtlasLayout>,
        Handle<AnimationClip2D>,
        bool,
    ) {
        match self {
            Attack::Light1 => (
                assets.arc.clone(),
                assets.arc_layout.clone(),
                assets.arc_animation.clone(),
                true,
            ),
            Attack::Light2 => (
                assets.arc.clone(),
                assets.arc_layout.clone(),
                assets.arc_animation.clone(),
                true,
            ),
            Attack::Light3 => (
                assets.arc.clone(),
                assets.arc_layout.clone(),
                assets.arc_animation.clone(),
                true,
            ),
            Attack::Heavy1 => (
                assets.vertical_line.clone(),
                assets.vertical_line_layout.clone(),
                assets.vertical_line_animation.clone(),
                false,
            ),
            Attack::Heavy2 => (
                assets.arc.clone(),
                assets.arc_layout.clone(),
                assets.arc_animation.clone(),
                true,
            ),
            Attack::Heavy3 => (
                assets.arc.clone(),
                assets.arc_layout.clone(),
                assets.arc_animation.clone(),
                true,
            ),
        }
    }
}
