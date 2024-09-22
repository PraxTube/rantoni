pub enum DudeAnimations {
    Idle,
    Run,
    Punch1,
    Punch1Recover,
    Punch2,
    Punch2Recover,
    StaggerNormal,
    Kick1,
    Kick1Recover,
    Kick2,
    Kick2Recover,
    Punch3,
    Punch3Recover,
    StaggerFlying,
    Kick3,
    Kick3Recover,
}

impl DudeAnimations {
    pub fn index(self) -> usize {
        self as usize
    }
}
