use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod camera;
mod combat;
mod enemy;
mod player;
mod states;
mod save_load;
mod gui;

use crate::{
    camera::CameraPlugin, combat::CombatPlugin, enemy::EnemyPlugin, player::PlayerPlugin,
    save_load::SaveLoadPlugin,states::Views, gui::GuiPlugin
};

fn main() {
    App::new()
        .add_state(Views::Combat)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(SaveLoadPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
