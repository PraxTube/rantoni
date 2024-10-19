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
