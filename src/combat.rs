// pub mod gui;

use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    enemy::Enemy,
    player::Player,
    save_load::{load_units, UnitJson},
};

pub struct CombatPlugin;

pub enum DamageType {
    Piercing,
    Bludgeoning,
    Slashing,
}

#[derive(Component)]
pub struct AttackReceive {
    pub hp: u32,
    pub max_hp: u32,
    pub weaknesses: Vec<DamageType>,
    pub resistances: Vec<DamageType>,
}

#[derive(Component)]
pub struct AttackSend {
    pub used: bool,
    pub dmg: u32,
    pub dmg_type: DamageType,
}

#[derive(Component)]
struct Highlight;

#[derive(Component)]
struct Active;

struct CombatEvent {}

fn parse_dmg_type(character: &char) -> DamageType {
    match character {
        'P' => DamageType::Piercing,
        'B' => DamageType::Bludgeoning,
        'S' => DamageType::Slashing,
        _ => DamageType::Slashing,
    }
}
fn spawn_team<T: Component + Copy>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    units: Vec<UnitJson>,
    team: T,
    x_offset: f32,
) {
    let mut i = -100.0;
    for unit in units.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(format!("sprites/{}.png", unit.sprite).as_str()),
                transform: Transform::from_translation(Vec3::new(x_offset, i, 1.0)),
                ..default()
            })
            .insert(team)
            .insert(AttackSend {
                used: false,
                dmg: unit.dmg,
                dmg_type: parse_dmg_type(&unit.dmg_type),
            })
            .insert(AttackReceive {
                hp: unit.hp,
                max_hp: unit.max_hp,
                weaknesses: unit.weaknesses.iter().map(parse_dmg_type).collect(),
                resistances: unit.resistances.iter().map(parse_dmg_type).collect(),
            });
        i += 100.0;
    }
}

fn spawn_highlight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/highlight.png"),
            transform: Transform::from_translation(Vec3::default()),
            ..default()
        })
        .insert(Highlight);
}
fn toggle_highlight(mut highlight_q: Query<&mut Visibility, With<Highlight>>) {
    let mut highlight = highlight_q.single_mut();
    highlight.is_visible = !highlight.is_visible;
}
fn move_highlight_to_active(
    mut highlight_q: Query<&mut Transform, (With<Highlight>, Without<Active>)>,
    active_q: Query<&Transform, (With<Active>, Without<Highlight>)>,
) {
    let mut highlight = highlight_q.single_mut();
    let active = active_q.single();
    highlight.translation = active.translation;
}
fn set_random_active_unit(mut commands: Commands, player_units: Query<Entity, With<Player>>) {
    let mut rng = rand::thread_rng();
    let player = player_units.iter().choose(&mut rng);
    if let Some(p) = player {
        commands.entity(p).insert(Active);
    }
}

fn spawn_teams(mut commands: Commands, asset_server: Res<AssetServer>) {
    let players = load_units("assets/players/team.json");
    spawn_team(&mut commands, &asset_server, players, Player, -300.0);

    let enemies = load_units("assets/encounters/3_pawns.json");
    spawn_team(&mut commands, &asset_server, enemies, Enemy, 300.0);
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_teams)
            .add_startup_system(set_random_active_unit)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_highlight)
            .add_startup_stage_after(StartupStage::PostStartup, "FinalStartup", SystemStage::parallel())
            .add_startup_system_to_stage("FinalStartup", move_highlight_to_active);
    }
}
