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
    Slide,
    Dropkick,
    Hammerfist,
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
    ///     - Position offset relative to the parent entity
    /// of the current attack.
    pub fn effect_position_data(&self) -> (Vec2, Collider, f32, Vec2) {
        match self {
            Attack::Light1 => (
                Vec2::default(),
                Collider::cuboid(8.0, 14.0),
                20.0,
                Vec2::ZERO,
            ),
            Attack::Light2 => (
                Vec2::default(),
                Collider::cuboid(8.0, 14.0),
                20.0,
                Vec2::ZERO,
            ),
            Attack::Light3 => (
                Vec2::default(),
                Collider::cuboid(12.0, 20.0),
                20.0,
                Vec2::ZERO,
            ),
            Attack::Heavy1 => (
                Vec2::default(),
                Collider::cuboid(12.0, 12.0),
                30.0,
                Vec2::ZERO,
            ),
            Attack::Heavy2 => (
                Vec2::default(),
                Collider::cuboid(8.0, 14.0),
                20.0,
                Vec2::ZERO,
            ),
            Attack::Heavy3 => (
                Vec2::default(),
                Collider::cuboid(8.0, 14.0),
                20.0,
                Vec2::ZERO,
            ),
            Attack::Slide => (
                Vec2::default(),
                Collider::cuboid(15.0, 8.0),
                20.0,
                Vec2::new(0.0, -16.0),
            ),
            Attack::Dropkick => (
                Vec2::default(),
                Collider::cuboid(10.0, 8.0),
                20.0,
                Vec2::new(0.0, -6.0),
            ),
            Attack::Hammerfist => (
                Vec2::default(),
                Collider::cuboid(8.0, 6.0),
                16.0,
                Vec2::new(0.0, -4.0),
            ),
        }
    }

    /// The bool flag is whether or not to rotate the arc attack effect locally, meaning on its z
    /// axis. It will always be rotated around the player regarding the orientation of the player.
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
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
            Attack::Light2 => (
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
            Attack::Light3 => (
                assets.attack_half_circle.clone(),
                assets.attack_half_circle_layout.clone(),
                assets.attack_half_circle_animation.clone(),
                true,
            ),
            Attack::Heavy1 => (
                assets.attack_flat_line.clone(),
                assets.attack_flat_line_layout.clone(),
                assets.attack_flat_line_animation.clone(),
                true,
            ),
            Attack::Heavy2 => (
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
            Attack::Heavy3 => (
                assets.attack_vertical_line.clone(),
                assets.attack_vertical_line_layout.clone(),
                assets.attack_vertical_line_animation.clone(),
                false,
            ),
            // TODO: Slide effect animations
            Attack::Slide => (
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
            // TODO: Dropkick effect animations
            Attack::Dropkick => (
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
            // TODO: hammerfist effect animations
            Attack::Hammerfist => (
                assets.attack_arc.clone(),
                assets.attack_arc_layout.clone(),
                assets.attack_arc_animation.clone(),
                true,
            ),
        }
    }
}
