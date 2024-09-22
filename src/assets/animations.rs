pub enum DudeAnimations {
    Idle,
    Run,
    Punch1,
    Punch1Recover,
    Punch2,
    Punch2Recover,
    Punch3,
    Punch3Recover,
    Kick1,
    Kick1Recover,
    Kick2,
    Kick2Recover,
    Kick3,
    Kick3Recover,
    StaggerNormal,
    StaggerFlying,
}

impl DudeAnimations {
    pub fn index(self) -> usize {
        self as usize
    }
}
