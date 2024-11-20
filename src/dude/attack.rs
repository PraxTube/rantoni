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
    Dropkick,
    Hammerfist,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AttackForm {
    #[default]
    None,
    Light,
    Heavy,
    SpecialLight,
    SpecialHeavy,
}

impl AttackForm {
    pub fn to_default_attack(self) -> Option<Attack> {
        match self {
            AttackForm::None => None,
            AttackForm::Light => Some(Attack::Light1),
            AttackForm::Heavy => Some(Attack::Heavy1),
            AttackForm::SpecialLight => Some(Attack::Hammerfist),
            AttackForm::SpecialHeavy => Some(Attack::Dropkick),
        }
    }
}

impl Attack {
    pub fn to_combo_attack(self, attack_form: AttackForm) -> Option<Attack> {
        match self {
            Attack::Light1 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => Some(Attack::Light2),
                AttackForm::Heavy => Some(Attack::Heavy1),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Light2 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => Some(Attack::Light3),
                AttackForm::Heavy => Some(Attack::Heavy3),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Light3 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => AttackForm::Light.to_default_attack(),
                AttackForm::Heavy => Some(Attack::Heavy2),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Heavy1 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => Some(Attack::Light2),
                AttackForm::Heavy => Some(Attack::Heavy2),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Heavy2 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => AttackForm::Light.to_default_attack(),
                AttackForm::Heavy => Some(Attack::Heavy3),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Heavy3 => match attack_form {
                AttackForm::None => None,
                AttackForm::Light => AttackForm::Light.to_default_attack(),
                AttackForm::Heavy => AttackForm::Heavy.to_default_attack(),
                AttackForm::SpecialLight => Some(Attack::Hammerfist),
                AttackForm::SpecialHeavy => Some(Attack::Dropkick),
            },
            Attack::Dropkick => None,
            Attack::Hammerfist => None,
        }
    }

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
