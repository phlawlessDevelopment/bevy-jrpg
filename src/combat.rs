// pub mod gui;

use bevy::{prelude::*, render::camera::RenderTarget};
use rand::seq::IteratorRandom;

use crate::{
    camera::MainCamera,
    enemy::Enemy,
    player::Player,
    save_load::{load_units, UnitJson},
    states::{Views, CombatPhases},
};

pub struct CombatPlugin;

#[derive(Debug, PartialEq, Eq)]
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

pub struct CombatEvent {
    pub send: Entity,
    pub receive: Entity,
}

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

fn spawn_highlight(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    mut phase: ResMut<State<CombatPhases>>,
) {
    if active_q.is_empty() && phase.overwrite_set(CombatPhases::SelectAction).is_ok() {}
    let mut highlight = highlight_q.single_mut();
    if let Some(active) = active_q.iter().next() {
        highlight.translation = active.translation;
        if phase.overwrite_set(CombatPhases::SelectAction).is_ok() {}
    }
}

fn set_random_active_unit(
    mut commands: Commands,
    player_units: Query<(Entity, &AttackSend), With<Player>>,
) {
    let mut rng = rand::thread_rng();
    let player = player_units
        .iter()
        .filter(|(_e, s)| !s.used)
        .choose(&mut rng);
    if let Some((e, _s)) = player {
        commands.entity(e).insert(Active);
    }
}

fn check_all_acted(
    player_units: Query<&AttackSend, With<Player>>,
    mut phase: ResMut<State<CombatPhases>>,
) {
    if player_units
        .iter()
        .filter(|s| !s.used)
        .peekable()
        .peek()
        .is_none()
        && phase.overwrite_set(CombatPhases::Enemy).is_ok()
    {}
}
fn check_all_dead(
    player_units: Query<&AttackSend, (With<Player>, Without<Enemy>)>,
    enemy_units: Query<&AttackSend, (With<Enemy>, Without<Player>)>,
    mut phase: ResMut<State<CombatPhases>>,
) {
    if player_units.iter().len() == 0 && phase.overwrite_set(CombatPhases::EnemyWins).is_ok() {
    } else if enemy_units.iter().len() == 0 && phase.overwrite_set(CombatPhases::PlayerWins).is_ok() {
    }
}

fn end_encounter() {
    println!("Encounter over");
}

fn player_wins() {
    println!("Player wins");
}

fn enemy_wins() {
    println!("Enemy wins");
}

fn remove_active_unit(mut commands: Commands, active: Query<Entity, With<Active>>) {
    if let Some(a) = active.iter().next() {
        commands.entity(a).remove::<Active>();
    }
}

fn listen_for_input(
    mut combat_event: EventWriter<CombatEvent>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Active>)>,
    active: Query<Entity, (With<Active>, Without<Enemy>)>,
    buttons: ResMut<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let _window = windows.get_primary().unwrap();
        let (camera, camera_transform) = q_camera.single();
        let wnd = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };
        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let mut world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            world_pos.z = 1.0;

            if let Some((receive, _transform)) = enemies
                .iter()
                .find(|(_e, t)| t.translation.distance(world_pos) <= 32.0)
            {
                if !active.is_empty() {
                    let send = active.single();
                    combat_event.send(CombatEvent { send, receive });
                }
            }
        }
    }
}

fn read_events(
    mut combat_events: EventReader<CombatEvent>,
    mut attacks: Query<(Entity, &mut AttackSend, &mut AttackReceive)>,
    mut commands: Commands,
    mut phase: ResMut<State<CombatPhases>>,
) {
    let mut sender: Option<Mut<AttackSend>> = Option::None;
    let mut receiver: Option<(Entity, Mut<AttackReceive>)> = Option::None;
    if let Some(event) = combat_events.iter().next() {
        for attack in attacks.iter_mut() {
            if attack.0 == event.send {
                sender = Some(attack.1);
            } else if attack.0 == event.receive {
                receiver = Some((attack.0, attack.2));
            }
        }
    }
    if let Some(mut s) = sender {
        if let Some(mut r) = receiver {
            let mut final_dmg = s.dmg;
            if r.1.resistances.contains(&s.dmg_type) {
                final_dmg /= 2;
            } else if r.1.resistances.contains(&s.dmg_type) {
                final_dmg *= 2;
            }
            r.1.hp = std::cmp::max(0, r.1.hp - final_dmg);
            println!("dmg: {}", final_dmg);
            println!("hp remaining: {}", r.1.hp);
            if r.1.hp == 0 {
                commands.entity(r.0).despawn_recursive();
                println!("dead");
            }
            s.used = true;
            if phase.overwrite_set(CombatPhases::SelectActive).is_ok() {}
        }
    }
}

fn clear_acted(mut sends: Query<&mut AttackSend>) {
    for mut send in sends.iter_mut() {
        send.used = false;
    }
}

fn do_enemy_turn(
    enemies: Query<Entity, (With<Enemy>, Without<Player>)>,
    players: Query<Entity, (With<Player>, Without<Enemy>)>,
    mut combat_event: EventWriter<CombatEvent>,
    mut phase: ResMut<State<CombatPhases>>,
) {
    let mut rng = rand::thread_rng();
    for send in enemies.iter() {
        if let Some(receive) = players.iter().choose(&mut rng) {
            combat_event.send(CombatEvent { send, receive });
        }
    }
    if phase.overwrite_set(CombatPhases::SelectActive).is_ok(){}
}

fn spawn_teams(mut commands: Commands, asset_server: Res<AssetServer>) {
    let players = load_units("assets/players/team.json");
    spawn_team(&mut commands, &asset_server, players, Player, -300.0);

    let enemies = load_units("assets/encounters/3_pawns.json");
    spawn_team(&mut commands, &asset_server, enemies, Enemy, 300.0);
}
fn start_encounter(
    mut phase: ResMut<State<CombatPhases>>,
    mut view: ResMut<State<Views>>
){  
    view.overwrite_set(Views::Combat).unwrap();
    phase.overwrite_set(CombatPhases::SelectActive).unwrap();
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(CombatPhases::SelectAction)
            .add_event::<CombatEvent>()
            .add_startup_system_to_stage(StartupStage::PreStartup, spawn_teams)
            .add_startup_system(set_random_active_unit)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_highlight)
            .add_startup_stage_after(
                StartupStage::PostStartup,
                "FinalStartup",
                SystemStage::parallel(),
            )
            .add_startup_system_to_stage("FinalStartup", move_highlight_to_active)
            .add_system_set(
                SystemSet::on_update(CombatPhases::SelectAction).with_system(listen_for_input),
            )
            .add_system_set(
                SystemSet::on_update(CombatPhases::SelectAction)
                    .with_system(check_all_acted)
                    .with_system(check_all_dead)
                    .with_system(read_events),
            )
            .add_system_set(
                SystemSet::on_enter(CombatPhases::SelectActive)
                    .with_system(remove_active_unit)
                    .with_system(set_random_active_unit.after(remove_active_unit)),
            )
            .add_system_set(
                SystemSet::on_update(CombatPhases::SelectActive)
                    .with_system(move_highlight_to_active)
                    .with_system(check_all_acted)
                    .with_system(check_all_dead),
            )
            .add_system_set(SystemSet::on_enter(CombatPhases::Enemy).with_system(toggle_highlight))
            .add_system_set(
                SystemSet::on_update(CombatPhases::Enemy)
                    .with_system(do_enemy_turn)
                    .with_system(check_all_dead),
            )
            .add_system_set(
                SystemSet::on_exit(CombatPhases::Enemy)
                    .with_system(toggle_highlight)
                    .with_system(clear_acted),
            )
            .add_system_set(SystemSet::on_enter(CombatPhases::EnemyWins).with_system(end_encounter))
            .add_system_set(SystemSet::on_update(CombatPhases::EnemyWins).with_system(enemy_wins))
            .add_system_set(
                SystemSet::on_enter(CombatPhases::PlayerWins).with_system(end_encounter),
            )
            .add_system_set(
                SystemSet::on_update(CombatPhases::PlayerWins).with_system(player_wins),
            );
    }
}
