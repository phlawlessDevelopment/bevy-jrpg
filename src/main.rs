use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use save_load::SaveLoadPlugin;

mod camera;
mod combat;
mod enemy;
mod player;
mod states;
mod save_load;

use crate::{
    camera::CameraPlugin, combat::CombatPlugin, enemy::EnemyPlugin, player::PlayerPlugin,
};

fn main() {
    App::new()
        // .add_state(Views::Combat)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(SaveLoadPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
