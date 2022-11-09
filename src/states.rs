#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Views {
    Combat,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum CombatPhases {
    SelectActive,
    SelectAction,
    SelectTarget,
    Enemy,
    EnemyWins,
    PlayerWins,
}
