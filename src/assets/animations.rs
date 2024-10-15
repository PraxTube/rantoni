#[derive(Default, Clone, Copy, PartialEq)]
pub enum DudeAnimations {
    #[default]
    Idle,
    Run,
    Light1,
    Light1Recover,
    Light2,
    Light2Recover,
    Light3,
    Light3Recover,
    StaggerNormal,
    Heavy1,
    Heavy1Recover,
    Heavy2,
    Heavy2Recover,
    Heavy3,
    Heavy3Recover,
    // TODO
    StaggerFlying,
}

impl DudeAnimations {
    pub fn index(self) -> usize {
        self as usize
    }
}
