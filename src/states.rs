#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Views {
    Combat,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum CombatPhases {
    SelectAction,
    SelectActive,
    Enemy,
    EnemyWins,
    PlayerWins,
}
