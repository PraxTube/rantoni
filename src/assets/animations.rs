pub enum DudeAnimations {
    Idle,
    Punch1,
    Punch2,
    Run,
    Punch1Recover,
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
