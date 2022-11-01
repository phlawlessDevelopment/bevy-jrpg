// pub mod gui;

use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Copy, Clone)]
pub struct Player;

fn setup() {}

fn teardown() {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
