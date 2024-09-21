mod attack;
mod stagger;

pub use attack::{Attack, AttackForm};
pub use stagger::{Stagger, StaggerState};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DudeState {
    #[default]
    Idling,
    Running,
    Attacking,
    Recovering,
}
