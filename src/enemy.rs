// pub mod gui;

use bevy::prelude::*;

pub struct EnemyPlugin;

#[derive(Component, Copy, Clone)]
pub struct Enemy;

fn setup() {}
fn teardown() {}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
